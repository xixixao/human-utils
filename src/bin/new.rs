use std::{io::Write, ops::Deref};

use camino::{Utf8Path, Utf8PathBuf};
use clap::{ArgAction, Args, Parser};
use colored::{ColoredString, Colorize};
use human_utils::{message_success, LazyPath, StandardOptions, FAILURE, SUCCESS};
use itertools::{Either, Itertools};

const SUCCESS_COLOR: colored::Color = colored::Color::BrightGreen;

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

const PATH_HELP: &str = const_format::formatcp!(
    "The path including the name of the new file or directory.
To create a directory end the PATH in {} or use the -d option",
    std::path::MAIN_SEPARATOR
);

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

    let (directory_paths, file_paths) = combine_input_paths(names);

    human_utils::set_color_override(&options);

    check_conflicts(&options, &directory_paths);

    for file_path in file_paths {
        let path = &Utf8Path::new(&file_path);
        human_utils::create_file(&options, path, None);
        print_success(&options, path, None);
    }

    for directory_path in directory_paths {
        let path = &Utf8Path::new(&directory_path);
        std::fs::create_dir_all(path).unwrap();
        print_success(&options, &human_utils::directory_path(path), None);
    }

    // if args.directory || args.path.ends_with(std::path::MAIN_SEPARATOR) {
    //     check_file_option_not_used(args, path);
    //     check_empty_directory_exists_already(args, path);
    //     // human_utils::check_path_exists_and_confirm_or_exit(options, path);
    //     // delete_directory_at_path(args, path);
    //     // let existing_ancestor = human_utils::find_existing_ancestor_directory(options, path);
    //     // human_utils::create_directory(options, path);
    //     // let paths = &join_paths(path, file_names);
    //     // create_files_with_content(args, paths);
    //     // print_directory_success(args, path, existing_ancestor);
    //     // print_files_success(args, paths, existing_ancestor);
    // } else {
    //     // let parent_directory = path.parent().unwrap();
    //     // check_directory_exists_and_confirm_or_exit(options, parent_directory);
    //     // let paths = &[
    //     //     vec![path.to_owned()],
    //     //     join_paths(parent_directory, file_names),
    //     // ]
    //     // .concat();
    //     // check_empty_files_exist_already(args, paths);
    //     // human_utils::check_paths_exist_and_confirm_or_exit(options, paths);
    //     // let existing_ancestor = human_utils::find_existing_ancestor_directory(options, path);
    //     // // delete_file_at_path(args, parent_directory);
    //     // delete_directories_at_paths(args, paths);
    //     // human_utils::create_directory(options, parent_directory);

    //     // create_files_with_content(args, paths);
    //     // print_files_success(args, paths, existing_ancestor);
    // };
    std::process::exit(SUCCESS);
}

// TODO: Get all the directories,
// and all the files
// and check that they are mutually exclusive
// otherwise do nothing and print error.
fn check_conflicts(options: &StandardOptions, directories: &Vec<String>) {
    let mut all_sources_already_at_destination = true;
    for directory_path in directories {
        if let Ok(metadata) = Utf8Path::new(directory_path).symlink_metadata() {
            let file_type = if metadata.is_dir() {
                "directory"
            } else {
                "file"
            };
            // #[tested(rem_basic)]
            // print!("Delete {} \"{}\"? [Y/n]", file_type, path);
        }
        // if source_parent.eq(&destination_canonical) {
        //     message_success!(
        //         options,
        //         "\"{}\" is already located at \"{}\"",
        //         source,
        //         human_utils::directory_path(destination)
        //     );
        // } else {
        //     all_sources_already_at_destination = false;
        // }
    }
    // all_sources_already_at_destination
}

fn combine_input_paths(names: Names) -> (Vec<String>, Vec<String>) {
    let Names {
        paths,
        file,
        directory,
    } = names;
    let (mut directory_paths, mut file_paths): (Vec<_>, Vec<_>) =
        paths.into_iter().partition_map(|path| {
            if path.ends_with(std::path::MAIN_SEPARATOR) {
                Either::Left(path.trim_end_matches(std::path::MAIN_SEPARATOR).to_string())
            } else {
                Either::Right(path)
            }
        });
    directory_paths.extend(directory);
    file_paths.extend(file);
    (directory_paths, file_paths)
}

// fn join_paths(parent: &Utf8Path, children: &Vec<String>) -> Vec<Utf8PathBuf> {
//     children.iter().map(|name| parent.join(name)).collect()
// }

// fn check_directory_exists_and_confirm_or_exit(
//     options: &human_utils::StandardOptions,
//     path: &Utf8Path,
// ) {
//     if options.force {
//         return;
//     }

//     if let Ok(metadata) = path.symlink_metadata() {
//         if !metadata.is_dir() {
//             print!(
//                 "A file \"{}\" exists, replace it with a directory? [Y/n]",
//                 path
//             );
//             human_utils::confirm_or_exit();
//         }
//     }
// }

// fn check_file_option_not_used(args: &Args, path: &LazyPath) {
//     if !args.file {
//         return;
//     }
//     eprintln!(
//         "Error: File path \"{}\" cannot end with a {} when --file option is used.",
//         path,
//         std::path::MAIN_SEPARATOR
//     );
//     std::process::exit(FAILURE);
// }

// fn check_empty_directory_exists_already(args: &Args, path: &mut LazyPath) {
//     if !args.content.is_empty() {
//         return;
//     }

//     if args.options.force {
//         return;
//     }

//     if let Ok(metadata) = path.metadata() {
//         if metadata.is_dir() && path.path.read_dir().unwrap().next().is_none() {
//             message_success!(
//                 args,
//                 "{}",
//                 format!("Empty directory \"{}\" already exists", path).color(SUCCESS_COLOR)
//             );
//             if args.file_names.is_empty() {
//                 std::process::exit(SUCCESS);
//             }
//         }
//     }
// }

// fn check_empty_files_exist_already(args: &Args, paths: &Vec<Utf8PathBuf>) {
//     if !args.content.is_empty() {
//         return;
//     }

//     if args.options.force {
//         return;
//     }

//     let mut all_exist_already = true;

//     for path in paths {
//         if let Ok(metadata) = path.symlink_metadata() {
//             if metadata.is_file() && metadata.len() == 0 {
//                 message_success!(
//                     args,
//                     "{}",
//                     format!("Empty file \"{}\" already exists", path).color(SUCCESS_COLOR)
//                 );
//                 continue;
//             }
//         }
//         all_exist_already = false;
//     }
//     if all_exist_already {
//         std::process::exit(SUCCESS);
//     }
// }

// fn delete_directories_at_paths(args: &Args, paths: &Vec<Utf8PathBuf>) {
//     if args.options.dry_run {
//         return;
//     }

//     for path in paths {
//         delete_directory_at_path(args, path);
//     }
// }

// fn delete_directory_at_path(args: &Args, path: &Utf8Path) {
//     if args.options.dry_run {
//         return;
//     }

//     if let Ok(metadata) = path.symlink_metadata() {
//         if metadata.is_dir() {
//             std::fs::remove_dir_all(path).unwrap();
//         }
//     }
// }

// fn create_files_with_content<Path: AsRef<Utf8Path>>(args: &Args, paths: &[Path]) {
//     if args.options.dry_run {
//         return;
//     }

//     let contents = args.content.join(" ");
//     for file_path in paths {
//         std::fs::write(file_path.as_ref(), &contents).unwrap();
//     }
// }

// fn print_directory_success(args: &Args, path: &Utf8Path, existing_ancestor: Option<&Utf8Path>) {
//     message_success!(
//         args,
//         "{}",
//         join_colored(
//             "N ".color(SUCCESS_COLOR),
//             human_utils::color_new(
//                 &human_utils::directory_path(path),
//                 existing_ancestor,
//                 SUCCESS_COLOR
//             )
//         )
//     );
// }

// fn print_files_success<Path: AsRef<Utf8Path>>(
//     args: &Args,
//     paths: &[Path],
//     existing_ancestor: Option<&Utf8Path>,
// ) {
//     if args.options.silent {
//         return;
//     }
//     for file_path in paths {
//         println!(
//             "{}",
//             join_colored(
//                 "N ".color(SUCCESS_COLOR),
//                 human_utils::color_new(file_path.as_ref(), existing_ancestor, SUCCESS_COLOR)
//             )
//         );
//     }
// }

// fn join_colored(first: ColoredString, second: ColoredString) -> ColoredString {
//     if let Some(color) = first.fgcolor() {
//         if Some(color) == second.fgcolor() {
//             return format!("{}{}", first.clear(), second.clear()).color(color);
//         }
//     }
//     format!("{}{}", first, second).normal()
// }

const COLOR: colored::Color = colored::Color::BrightGreen;

fn print_success(
    options: &StandardOptions,
    created: &Utf8Path,
    existing_ancestor: Option<&Utf8Path>,
) {
    message_success!(
        options,
        "{} {}",
        "N".color(COLOR),
        human_utils::color_new(created, existing_ancestor, COLOR)
    );
}
