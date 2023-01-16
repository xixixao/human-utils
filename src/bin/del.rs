use camino::Utf8Path;
use clap::Parser;
use colored::*;
use human_utils::{confirm_or_exit, message_success, path_string, FAILURE, SUCCESS};

// TODO: Support `del .` and `del ..`

const DETAILS: &str = "
As part of `human-utils`, `del` asks for confirmation before
deleting any <FILE_OR_DIRECTORY>.

Examples where `del` differs from `rm`:

  Asks for confirmation:
    `del a` where a is an existing file will
    ask for a confirmation and then will delete `a`,
    while `rm` will irreversibly delete `a`
    without any confirmation.

  Always deletes:
    `del a` where a is an existing directory,
    `del` will ask for confirmation and then
    will delete `a`, while `rm a`
    will error out and request the use of `-r` option.

Exits with non-zero (failure) value if no files/directories were
removed or if some existing files/directories were not removed.
";

/// `del`ete files and directories
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct CLI {
    /// The paths to one or more files/directories
    // #[tested(rem_requires_at_least_one_argument)]
    #[arg(required(true))]
    file_or_directory: Vec<String>,

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

fn main() {
    let args = CLI::parse();
    let paths = args.file_or_directory.iter().map(Utf8Path::new).collect();
    human_utils::set_color_override(&args.options);
    ask_to_confirm(&args, &paths);
    let all_removed = remove(&args, &paths);
    std::process::exit(if all_removed { SUCCESS } else { FAILURE });
}

fn ask_to_confirm(args: &CLI, paths: &Vec<&Utf8Path>) {
    if args.options.force {
        return;
    }

    let is_single = paths.len() == 1;
    if is_single {
        let path = paths.first().unwrap();
        ask_for_single_path(&path);
    } else {
        ask_for_multiple_paths(&paths);
    }

    // #[tested(rem_basic)]
    confirm_or_exit();
}

fn ask_for_single_path(path: &Utf8Path) {
    match path.symlink_metadata() {
        Ok(metadata) => {
            let file_type = if metadata.is_dir() {
                "directory"
            } else {
                "file"
            };
            // #[tested(rem_basic)]
            print!("Delete {} \"{}\"? [Y/n]", file_type, path);
        }
        Err(error) => {
            // #[tested(rem_no_existing::nonexistent_path_fails)]
            eprintln!("Error for \"{}\": {}", path, error);
            std::process::exit(FAILURE);
        }
    }
}

fn ask_for_multiple_paths(paths: &Vec<&Utf8Path>) {
    println!("For the following...");
    let mut all_exist = true;
    let mut some_exist = false;
    for path in paths {
        let metadata = path.symlink_metadata();
        print_path(path, &metadata);
        let exists = metadata.is_ok();
        all_exist = all_exist && exists;
        some_exist = some_exist || exists;
    }
    if !some_exist {
        // #[tested(rem_no_existing)]
        eprintln!("...no files or directories can be removed.");
        std::process::exit(FAILURE);
    }
    // #[tested(rem_basic)]
    print!(
        "...delete all{}? [Y/n]",
        if all_exist { "" } else { " existing" }
    );
}

fn print_path(path: &Utf8Path, metadata: &std::io::Result<std::fs::Metadata>) {
    match metadata {
        Ok(metadata) => {
            // #[tested(rem_multiple)]
            if metadata.is_dir() {
                println!("{}", path_string(&format!("{}/", path)));
            } else {
                println!("{}", path_string(path));
            }
        }
        Err(error) => {
            // #[tested(rem_no_existing)]
            eprintln!("\"{}\" error: {}", path, error);
        }
    }
}

// #[tested(TODO)]
fn remove(args: &CLI, paths: &Vec<&Utf8Path>) -> bool {
    let mut all_removed = true;
    for path in paths {
        let removed = if let Ok(metadata) = path.symlink_metadata() {
            if metadata.is_dir() {
                remove_dir(args, path)
            } else {
                remove_file(args, path)
            }
        } else {
            // Consider non-existing files and directories as removed
            true
        };
        if !removed {
            all_removed = false;
        }
    }
    all_removed
}

fn remove_dir(args: &CLI, path: &Utf8Path) -> bool {
    if !args.options.dry_run {
        // #[tested(rem_basic)]
        if let Err(error) = std::fs::remove_dir_all(path) {
            // #[tested(TODO)]
            eprintln!("Error for directory \"{}\": {}", path, error);
            return false;
        }
    }
    message_success!(
        args,
        "{}",
        format!("D {}", path_string(format!("{}/", path))).bright_red()
    );
    true
}

fn remove_file(args: &CLI, path: &Utf8Path) -> bool {
    if !args.options.dry_run {
        // #[tested(rem_basic)]
        if let Err(error) = std::fs::remove_file(path) {
            // #[tested(TODO)]
            eprintln!("Error for file \"{}\": {}", path, error);
            return false;
        }
    }
    message_success!(args, "{}", format!("D {}", path_string(path)).bright_red());
    true
}
