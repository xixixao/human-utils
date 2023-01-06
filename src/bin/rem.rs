use camino::Utf8Path;
use clap::Parser;
use colored::*;
use human_utils::{confirm_or_exit, message_success, quote, FAILURE, SUCCESS};

// TODO: Support `rem .` and `rem ..`

const DETAILS: &str = "
As part of `human-utils`, `rem` asks for confirmation before
removing any <FILE_OR_DIRECTORY>.

Examples where `rem` differs from `rm`:

  Asks for confirmation:
    `rem a` where a is an existing file will
    ask for a confirmation and then will remove `a`,
    while `rm` will irreversibly remove `a`
    without any confirmation.

  Always removes:
    `rem a` where a is an existing directory,
    `rem` will ask for confirmation and then
    will remove `a`, while `rm a`
    will error out and request the use of `-r` option.

Exits with non-zero (failure) value if no files/directories were
removed or if some existing files/directories were not removed.
";

/// `rem`ove files and directories
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct CLI {
    /// The paths to one or more files/directories
    // #[tested(rem_requires_at_least_one_argument)]
    #[arg(required(true))]
    file_or_directory: Vec<String>,

    /// Never ask for confirmation
    #[arg(short, long)]
    // #[tested(rem_force)]
    force: bool,

    /// Do not print success messages, still prints errors
    #[arg(short, long)]
    // #[tested(rem_silent)]
    silent: bool,

    /// Ask for confirmation, print success messages and errors, but do not perform any changes
    #[arg(short = 'n', long)]
    // #[tested(rem_dry_run)]
    dry_run: bool,
}

fn main() {
    let args = CLI::parse();
    let paths = args.file_or_directory.iter().map(Utf8Path::new).collect();
    ask_to_confirm(&args, &paths);
    let all_removed = remove(&args, &paths);
    print_outcome(&args, all_removed);
    std::process::exit(SUCCESS);
}

fn ask_to_confirm(args: &CLI, paths: &Vec<&Utf8Path>) {
    if args.force {
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
            print!("Remove {} \"{}\"? [Y/n]", file_type, path);
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
        "...remove all{}? [Y/n]",
        if all_exist { "" } else { " existing" }
    );
}

fn print_path(path: &Utf8Path, metadata: &std::io::Result<std::fs::Metadata>) {
    match metadata {
        Ok(metadata) => {
            // #[tested(rem_multiple)]
            if metadata.is_dir() {
                println!("{}", quote(&format!("{}/", path.as_str().blue())));
            } else {
                println!("{}", quote(path.as_str()));
            }
        }
        Err(error) => {
            // #[tested(rem_no_existing)]
            eprintln!("{} error: {}", quote(path.as_str()), error);
        }
    }
}

// #[tested(TODO)]
fn remove(args: &CLI, paths: &Vec<&Utf8Path>) -> bool {
    if args.dry_run {
        return true;
    }
    let mut all_removed = true;
    for path in paths {
        let removed = if let Ok(metadata) = path.symlink_metadata() {
            if metadata.is_dir() {
                remove_dir(path)
            } else {
                remove_file(path)
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

fn remove_dir(path: &Utf8Path) -> bool {
    // #[tested(rem_basic)]
    if let Err(error) = std::fs::remove_dir_all(path) {
        // #[tested(TODO)]
        eprintln!("Error for directory \"{}\": {}", path, error);
        return false;
    }
    true
}

fn remove_file(path: &Utf8Path) -> bool {
    // #[tested(rem_basic)]
    if let Err(error) = std::fs::remove_file(path) {
        // #[tested(TODO)]
        eprintln!("Error for file \"{}\": {}", path, error);
        return false;
    }
    true
}

fn print_outcome(args: &CLI, all_removed: bool) {
    // #[tested(TODO)]
    if !all_removed {
        eprintln!("Error: Some existing files or directories could not be deleted");
        std::process::exit(FAILURE);
    }
    // #[tested(rem_basic, rem_silent)]
    message_success!(args, "Done");
}
