use camino::Utf8Path;
use clap::Parser;
use colored::Colorize;
use human_utils::{message_success, path_string, FAILURE, SUCCESS};

// TODO: Support `ren . something_else` and `ren something_else .`

const DETAILS: &str = "
As part of `human-utils`, `ren` asks for confirmation if
a file or directory already exists at <DESTINATION>.

Examples where `ren` differs from `mv`:

  Asks for confirmation:
    `ren a b` where b is an existing file,
    `ren` will ask for confirmation and then
    will replace `b` with `a`, while `mv`
    will irreversibly replace `b` with `a`
    without any confirmation.

  Always renames:
    `ren a b` where b is an existing directory,
    `ren` will ask for confirmation and then
    will replace `b` with `a`, while `mv`
    will move `a` into the directory `b`

Other improvements:
  
  Existing location:
    `ren a a` will return success code 0,
    while `mv` will return error code 1.

  Existing location with different path format:
    `ren a /foo/a` where `a` is already located at `/foo/a`,
    `ren` will note that `a` is already located at `/foo/a`
    and return success code 0, while `mv` will consider this
    a valid move and `mv -i` will ask for confirmation.

";

/// `ren`e files and directories
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

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

fn main() {
    let args = &CLI::parse();
    let options = &args.options;
    let source = Utf8Path::new(&args.file_or_directory);
    let destination = Utf8Path::new(&args.destination);
    human_utils::set_color_override(&args.options);
    check_source_exists(source);
    check_source_already_at_destination(args, source, destination);
    check_destination_exists(args, destination);
    let existing_ancestor = human_utils::find_existing_ancestor_directory(options, destination);
    human_utils::create_parent_directory(options, destination);
    rename(args, source, destination);
    print_success(args, destination, existing_ancestor);
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
    if args.options.force {
        return;
    }
    human_utils::check_path_exists_and_confirm_or_exit(destination);
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
    if args.options.dry_run {
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

const COLOR: colored::Color = colored::Color::BrightGreen;

// #[tested(nam_basic, nam_silent)]
fn print_success(args: &CLI, destination: &Utf8Path, existing_ancestor: Option<&Utf8Path>) {
    message_success!(
        args,
        "{} {} -> {}",
        "R".color(COLOR),
        path_string(&args.file_or_directory).bright_red(),
        human_utils::color_new(destination, existing_ancestor, COLOR)
    );
}
