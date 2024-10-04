use std::collections::{BTreeMap, BTreeSet};

use camino::{Utf8Path, Utf8PathBuf};
use clap::{ArgAction, Args, Parser};
use colored::Colorize;
use human_utils::{message_success, StandardOptions, FAILURE, SUCCESS};
use itertools::{Either, Itertools};

const DETAILS: &str = "
Basic examples:

  Create a file with some text:
    `new path/to/new_file.txt -- Hello world`

  Create a directory:
    `new path/to/new_dir/`

  Create a directory with 3 empty files:
    `new path/to/new_dir/{a,b,c}`

As part of `human-utils`, `new` asks for confirmation before\
overwriting any file or directory.";

const FILE_HELP: &str = const_format::formatcp!(
    "Relative or absolute path of a file to create. Errors if FILE ends in {}",
    std::path::MAIN_SEPARATOR
);

/// `new` - create new files or directories
#[derive(Parser)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct CLI {
    #[command(flatten)]
    names: Names,

    /// The string content of the new file(s).
    /// If not provided, the new file(s) will be empty
    #[arg(last(true))]
    content: Vec<String>,

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

// TODO: Ideally we'd preserve the original order
// of arguments instead of grouping them, but that's
// more complicated with `clap`
#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct Names {
    /// Relative or absolute paths of files and directories to create.
    paths: Vec<String>,

    #[arg(long, help(FILE_HELP), action(ArgAction::Append))]
    file: Vec<String>,

    /// Relative or absolute path of a directory to create.
    #[arg(short, long, action(ArgAction::Append))]
    directory: Vec<String>,
}

fn main() {
    let CLI {
        names,
        content,
        options,
    } = CLI::parse();

    let (all_directory_paths, directory_paths, file_paths) = combine_input_paths(names);

    human_utils::set_color_override(&options);

    check_argument_conflicts(&all_directory_paths, &file_paths);

    let (clashing_with_directories, clashing_with_files) =
        check_conflicts(&options, &all_directory_paths, &file_paths);

    delete_clashing(&options, &clashing_with_directories, &clashing_with_files);

    for path in directory_paths {
        let clashing = clashing_with_directories.get(&path);
        if clashing.map_or(false, |metadata| metadata.is_dir()) {
            message_success!(options, "Directory \"{}\" already exists", path);
            continue;
        }
        if !options.dry_run {
            std::fs::create_dir_all(&path).unwrap();
        }
        print_success(
            &options,
            &human_utils::directory_path(&path),
            None,
            find_existing_ancestor_directory_for_printout(
                &options,
                &path,
                &clashing_with_directories,
            ),
        );
    }

    let content = if content.is_empty() {
        None
    } else if content.len() == 1 && content[0] == "" {
        None
    } else {
        Some(content.join(" ") + "\n")
    };
    for path in file_paths {
        let clashing = clashing_with_files.get(&path);
        if content.is_none()
            && clashing.map_or(false, |metadata| !metadata.is_dir() && metadata.len() == 0)
        {
            message_success!(options, "File \"{}\" already exists", path);
            continue;
        }
        human_utils::create_file(&options, &path, content.as_deref());
        print_success(
            &options,
            &path,
            clashing,
            find_existing_ancestor_directory_for_printout(
                &options,
                &path,
                &clashing_with_directories,
            ),
        );
    }
    std::process::exit(SUCCESS);
}

fn delete_clashing(
    options: &StandardOptions,
    clashing_with_directories: &BTreeMap<Utf8PathBuf, std::fs::Metadata>,
    clashing_with_files: &BTreeMap<Utf8PathBuf, std::fs::Metadata>,
) {
    for (path, metadata) in clashing_with_directories {
        if !metadata.is_dir() {
            message_success!(
                options,
                "{}",
                format!("D {}", human_utils::format_path(path, metadata)).bright_red()
            );
            if options.dry_run {
                continue;
            }
            std::fs::remove_file(path).unwrap();
        }
    }
    for (path, metadata) in clashing_with_files {
        if metadata.is_dir() {
            message_success!(
                options,
                "{}",
                format!("D {}", human_utils::format_path(path, metadata)).bright_red()
            );
            if options.dry_run {
                continue;
            }
            std::fs::remove_dir_all(path).unwrap();
        }
    }
}

fn check_argument_conflicts(
    all_directory_paths: &BTreeSet<Utf8PathBuf>,
    file_paths: &BTreeSet<Utf8PathBuf>,
) {
    let clashing: BTreeSet<_> = all_directory_paths.intersection(file_paths).collect();

    if !clashing.is_empty() {
        eprintln!(
            "Error: Cannot create both file and a directory at:\n{}",
            clashing.into_iter().join("\n")
        );
        std::process::exit(FAILURE);
    }
}

fn combine_input_paths(
    names: Names,
) -> (
    BTreeSet<Utf8PathBuf>,
    BTreeSet<Utf8PathBuf>,
    BTreeSet<Utf8PathBuf>,
) {
    let Names {
        paths,
        file,
        directory,
    } = names;
    for path in &file {
        if path.ends_with(std::path::MAIN_SEPARATOR) {
            eprintln!(
                "Error: File path \"{}\" cannot end with a `{}` when `--file` option is used.",
                path,
                std::path::MAIN_SEPARATOR
            );
            std::process::exit(FAILURE);
        }
    }
    let (mut directory_path_strings, mut file_path_strings): (BTreeSet<_>, BTreeSet<_>) =
        paths.into_iter().partition_map(|path| {
            if path.ends_with(std::path::MAIN_SEPARATOR) {
                Either::Left(path.trim_end_matches(std::path::MAIN_SEPARATOR).to_string())
            } else {
                Either::Right(path)
            }
        });
    directory_path_strings.extend(directory);
    file_path_strings.extend(file);
    let directory_paths: BTreeSet<_> = directory_path_strings
        .iter()
        .map(Utf8PathBuf::from)
        .collect();
    let file_paths: BTreeSet<_> = file_path_strings.iter().map(Utf8PathBuf::from).collect();

    let all_directory_paths: BTreeSet<_> = directory_paths
        .iter()
        .flat_map(|path| path.ancestors())
        .map(|path| path.to_owned())
        .chain(file_paths.iter().flat_map(|path| {
            path.parent()
                .map(|parent| parent.ancestors())
                .into_iter()
                .flatten()
                .map(|path| path.to_owned())
        }))
        .filter(|path| path != "")
        .collect();

    (all_directory_paths, directory_paths, file_paths)
}

fn check_conflicts(
    options: &StandardOptions,
    all_directory_paths: &BTreeSet<Utf8PathBuf>,
    file_paths: &BTreeSet<Utf8PathBuf>,
) -> (
    BTreeMap<Utf8PathBuf, std::fs::Metadata>,
    BTreeMap<Utf8PathBuf, std::fs::Metadata>,
) {
    let clashing_with_directories = all_directory_paths
        .iter()
        .filter_map(|path| {
            path.symlink_metadata()
                .ok()
                .map(|metadata| (path.clone(), metadata))
        })
        .collect::<BTreeMap<_, _>>();

    let clashing_with_files = file_paths
        .iter()
        .filter_map(|path| {
            path.symlink_metadata()
                .ok()
                .map(|metadata| (path.clone(), metadata))
        })
        .collect::<BTreeMap<_, _>>();

    if options.force {
        return (clashing_with_directories, clashing_with_files);
    }

    human_utils::ask_to_overwrite(
        &clashing_with_directories
            .iter()
            .filter(|(_, metadata)| !metadata.is_dir())
            .map(|(path, _)| path)
            .chain(
                clashing_with_files
                    .iter()
                    .filter(|(_, metadata)| metadata.is_dir() || metadata.len() > 0)
                    .map(|(path, _)| path),
            )
            .collect(),
    );

    (clashing_with_directories, clashing_with_files)
}

pub fn find_existing_ancestor_directory_for_printout<'a>(
    options: &StandardOptions,
    path: &'a Utf8Path,
    existing_ancestors: &BTreeMap<Utf8PathBuf, std::fs::Metadata>,
) -> Option<&'a Utf8Path> {
    if options.no_color {
        return None;
    }

    let mut ancestor = path.parent();
    while let Some(ancestor_path) = ancestor {
        if existing_ancestors
            .get(ancestor_path)
            .map_or(false, |metadata| metadata.is_dir())
        {
            return Some(ancestor_path);
        }
        ancestor = ancestor_path.parent();
    }
    None
}

const COLOR: colored::Color = colored::Color::BrightGreen;

fn print_success(
    options: &StandardOptions,
    created: &Utf8Path,
    clashing: Option<&std::fs::Metadata>,
    existing_ancestor: Option<&Utf8Path>,
) {
    let did_overwite_file = clashing.map_or(false, |metadata| !metadata.is_dir());
    message_success!(
        options,
        "{} {}",
        (if did_overwite_file { "M" } else { "N" }).color(COLOR),
        human_utils::color_new(created, existing_ancestor, COLOR)
    );
}
