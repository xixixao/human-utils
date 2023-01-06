use camino::Utf8Path;
use clap::Parser;
use human_utils::{confirm_or_exit, message_success, FAILURE, SUCCESS};

// TODO: Support `nam . something_else` and `nam something_else .`

const DETAILS: &str = "
As part of `human-utils`, `nam` asks for confirmation if
a file or directory already exists at <DESTINATION>.

Examples where `nam` differs from `mv`:

  Asks for confirmation:
    `nam a b` where b is an existing file,
    `nam` will ask for confirmation and then
    will replace `b` with `a`, while `mv`
    will irreversibly replace `b` with `a`
    without any confirmation.

  Always renames:
    `nam a b` where b is an existing directory,
    `nam` will ask for confirmation and then
    will replace `b` with `a`, while `mv`
    will move `a` into the directory `b`

Other improvements:
  
  Existing location:
    `nam a a` will return success code 0,
    while `mv` will return error code 1.

  Existing location with different path format:
    `nam a /foo/a` where `a` is already located at `/foo/a`,
    `nam` will note that `a` is already located at `/foo/a`
    and return success code 0, while `mv` will consider this
    a valid move and `mv -i` will ask for confirmation.

";

/// `nam`e files and directories
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct CLI {
    /// The path to the source file or directory
    // #[tested(nam_requires_two_arguments)]
    file_or_directory: String,

    /// The new path the file_or_directory should live at
    // #[tested(nam_requires_two_arguments)]
    destination: String,

    /// Never ask for confirmation
    #[arg(short, long)]
    // #[tested(nam_force)]
    force: bool,

    /// Do not print success messages, still prints errors
    #[arg(short, long)]
    // #[tested(nam_silent)]
    silent: bool,

    /// Ask for confirmation, print success messages and errors, but do not perform any changes
    #[arg(short = 'n', long)]
    // #[tested(nam_dry_run)]
    dry_run: bool,
}

fn main() {
    let args = CLI::parse();
    let source = Utf8Path::new(&args.file_or_directory);
    let destination = Utf8Path::new(&args.destination);
    check_source_exists(&source);
    check_source_already_at_destination(&args, &source, &destination);
    check_destination_exists(&args, &destination);
    rename(&args, &source, &destination);
    print_success(&args);
    std::process::exit(SUCCESS);
}

// #[tested(nam_missing_source)]
fn check_source_exists(source: &Utf8Path) {
    match source.symlink_metadata() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error for \"{}\": {}", source, error);
            std::process::exit(FAILURE);
        }
    }
}

// #[tested(nam_noop, nam_silent)]
fn check_source_already_at_destination(args: &CLI, source: &Utf8Path, destination: &Utf8Path) {
    match (source.canonicalize_utf8(), destination.canonicalize_utf8()) {
        (Ok(canonical_source), Ok(canonical_destination)) => {
            if canonical_source.eq(&canonical_destination) {
                message_success!(
                    args,
                    "\"{}\" is already located at \"{}\"",
                    source,
                    destination
                );
                std::process::exit(SUCCESS);
            }
        }
        (_, _) => {}
    }
}

// #[tested(nam_replace_existing)]
fn check_destination_exists(args: &CLI, destination: &Utf8Path) {
    if args.force {
        return;
    }

    if let Ok(metadata) = destination.symlink_metadata() {
        let file_type = if metadata.is_dir() {
            "Directory"
        } else {
            "File"
        };
        print!(
            "{} \"{}\" already exists, replace it? [Y/n]",
            file_type, destination
        );
        confirm_or_exit();
    }
}

// #[tested(nam_basic, nam_replace_existing)]
fn rename(args: &CLI, from: &Utf8Path, to: &Utf8Path) {
    // TODO: Make sure this works correctly on Windows
    // if to.exists() && to.is_dir() {
    //     // normalize behavior between *nix and windows
    //     if from.is_dir()
    //             fs::remove_dir(to)?;
    //     }
    // }
    if args.dry_run {
        return;
    }

    if std::fs::rename(from, to).is_err() {
        if to.exists() {
            if to.is_dir() {
                std::fs::remove_dir_all(to).unwrap();
            } else {
                std::fs::remove_file(to).unwrap();
            }
        }
        std::fs::rename(from, to).unwrap();
    }
}

// #[tested(nam_basic, nam_silent)]
fn print_success(args: &CLI) {
    message_success!(
        args,
        "\"{}\" -> \"{}\"",
        args.file_or_directory,
        args.destination
    );
}
