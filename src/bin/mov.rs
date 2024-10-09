use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use colored::Colorize;
use human_utils::{message_success, path_string, StandardOptions, FAILURE, SUCCESS};

// TODO: Support `mov . something_else` and `mov something_else .`

const DETAILS: &str = "
As part of `human-utils`, `mov` asks for confirmation if
a file or directory already exists at <DESTINATION>.

Examples where `mov` differs from `mv`:

  Asks for confirmation:
    `mov a b` where b is an existing file,
    `mov` will ask for a confirmation and then
    will replace `b` with `a`, while `mv`
    will irreversibly replace `b` with `a`
    without any confirmation.

  Always renames:
    `mov a b` where b is an existing directory,
    `mov` will ask for confirmation and then
    will replace `b` with `a`, while `mv`
    will move `a` into the directory `b`

Other improvements:
  
  Existing location:
    `mov a a` will return success code 0,
    while `mv` will return error code 1.

  Existing location with different path format:
    `mov a /foo/a` where `a` is already located at `/foo/a`,
    `mov` will note that `a` is already located at `/foo/a`
    and return success code 0, while `mv` will consider this
    a valid move and `mv -i` will ask for confirmation.

";

const DESTINATION_HELP: &str = const_format::formatcp!(
    "The new path the moved files or directories should live at.
To move the files or directories to a directory, end the DESTINATION_PATH in {} or use the -m option",
    std::path::MAIN_SEPARATOR
);

/// `mov`e files and directories
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct CLI {
    /// The paths of the files or directories to be moved
    #[arg(required(true))]
    source_paths: Vec<String>,

    #[arg(help(DESTINATION_HELP))]
    destination_path: String,

    // TODO: Consider renaming to --directory
    // TODO: Change to option with argument
    /// Move files or directories into a directory at DESTINATION_PATH.
    #[arg(short, long)]
    move_into: bool,

    // TODO: Consider renaming to --file
    // TODO: Change to option with argument
    /// Rename and move one file or directory from SOURCE_PATH to DESTINATION_PATH.
    #[arg(short, long)]
    rename: bool,

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

fn main() {
    let args = &CLI::parse();
    let options = &args.options;
    let sources: &Vec<_> = &args.source_paths.iter().map(Utf8Path::new).collect();
    let destination = Utf8Path::new(&args.destination_path);
    human_utils::set_color_override(&args.options);
    if args.move_into || args.destination_path.ends_with(std::path::MAIN_SEPARATOR) {
        check_sources_exists(sources);
        check_sources_already_at_destination(options, sources, destination);
        let paths_at_destination = &get_paths_at_destination(sources, destination);
        human_utils::check_paths_exist_and_confirm_or_exit(options, paths_at_destination);
        let existing_ancestor = human_utils::find_existing_ancestor_directory(options, destination);
        human_utils::create_directory(options, destination);
        rename_all(options, sources, paths_at_destination);
        print_success_all(options, sources, paths_at_destination, existing_ancestor);
    } else {
        let source = only_one_source(args, sources);
        let canonical_source = check_source_exists(source);
        check_source_already_at_destination(options, source, &canonical_source, destination);
        human_utils::check_path_exists_and_confirm_or_exit(options, destination);
        let existing_ancestor = human_utils::find_existing_ancestor_directory(options, destination);
        human_utils::create_parent_directory(options, destination);
        rename(options, source, destination);
        print_success(options, source, destination, existing_ancestor);
    }
    std::process::exit(SUCCESS);
}

// TODO: Redo via options
fn only_one_source<'a>(args: &CLI, sources: &'a Vec<&'a Utf8Path>) -> &'a Utf8Path {
    if sources.len() != 1 {
        eprintln!(
            "Error: Expected 1 SOURCE_PATH argument because {}, but got {}",
            if args.move_into {
                "the --move-into option was used".to_owned()
            } else {
                format!(
                    "DESTINATION_PATH did not end with a {}",
                    std::path::MAIN_SEPARATOR
                )
            },
            sources.len()
        );
        std::process::exit(FAILURE);
    }
    sources[0]
}

fn check_sources_exists(sources: &Vec<&Utf8Path>) {
    for source in sources {
        check_source_exists(source);
    }
}

fn check_source_exists(source: &Utf8Path) -> Utf8PathBuf {
    match source.canonicalize_utf8() {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Error for \"{}\": {}", source, error);
            std::process::exit(FAILURE);
        }
    }
}

fn check_sources_already_at_destination(
    options: &StandardOptions,
    sources: &Vec<&Utf8Path>,
    destination: &Utf8Path,
) {
    let destination_canonical = destination.canonicalize_utf8().unwrap();
    let mut all_sources_already_at_destination = true;
    for source in sources {
        let source_canonical = source.canonicalize_utf8().unwrap();
        let source_parent = source_canonical.parent().unwrap();
        if source_parent.eq(&destination_canonical) {
            message_success!(
                options,
                "\"{}\" is already located at \"{}\"",
                source,
                human_utils::directory_path(destination)
            );
        } else {
            all_sources_already_at_destination = false;
        }
    }
    if all_sources_already_at_destination {
        std::process::exit(SUCCESS);
    }
}

fn check_source_already_at_destination(
    options: &StandardOptions,
    source: &Utf8Path,
    canonical_source: &Utf8Path,
    destination: &Utf8Path,
) {
    if let Ok(canonical_destination) = destination.canonicalize_utf8() {
        if canonical_source == canonical_destination {
            message_success!(
                options,
                "\"{}\" is already located at \"{}\"",
                source,
                destination
            );
            std::process::exit(SUCCESS);
        }
    }
}

fn get_paths_at_destination(sources: &Vec<&Utf8Path>, destination: &Utf8Path) -> Vec<Utf8PathBuf> {
    sources
        .iter()
        .map(|source| destination.join(source.file_name().unwrap()))
        .collect()
}

fn rename_all(
    options: &StandardOptions,
    sources: &Vec<&Utf8Path>,
    paths_at_destination: &Vec<Utf8PathBuf>,
) {
    for (i, source) in sources.iter().enumerate() {
        let destination_path = &paths_at_destination[i];
        rename(options, source, &destination_path);
    }
}

fn rename(options: &StandardOptions, from: &Utf8Path, to: &Utf8Path) {
    // TODO: Make sure this works correctly on Windows
    // if to.exists() && to.is_dir() {
    //     // normalize behavior between *nix and windows
    //     if from.is_dir()
    //             fs::remove_dir(to)?;
    //     }
    // }
    if options.dry_run {
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

fn print_success_all(
    options: &StandardOptions,
    sources: &Vec<&Utf8Path>,
    paths_at_destination: &Vec<Utf8PathBuf>,
    existing_ancestor: Option<&Utf8Path>,
) {
    for (i, source) in sources.iter().enumerate() {
        let destination_path = &paths_at_destination[i];
        print_success(options, source, destination_path, existing_ancestor);
    }
}

fn print_success(
    options: &StandardOptions,
    source: &Utf8Path,
    destination: &Utf8Path,
    existing_ancestor: Option<&Utf8Path>,
) {
    message_success!(
        options,
        "{} {} -> {}",
        "M".color(COLOR),
        path_string(source).bright_red(),
        human_utils::color_new(destination, existing_ancestor, COLOR)
    );
}
