// // Example of bizarre cp behavior:
// // `cp -r foo/bla/ foo/` inlines files from `bla` in `foo`, instead of pointing out that foo is already in bla
// // ^ that's simply because cp already differs behavior on trailing slash, in this case copying contents (as if * was used)

// use camino::{Utf8Path, Utf8PathBuf};
// use clap::{CommandFactory, Parser};
// use colored::Colorize;
// use human_utils::{message_success, path_string, StandardOptions, FAILURE, SUCCESS};

// const DETAILS: &str = "
// As part of `human-utils`, `cop` asks for confirmation if
// a file or directory already exists at <DESTINATION_PATH>.

// Examples where `cop` differs from `cp`:

//   Asks for confirmation:
//     `cop a b` where b is an existing file,
//     `cop` will ask for a confirmation and then
//     will replace `b` with a copy of `a`, while `cp`
//     will irreversibly replace `b` with a copy of `a`
//     without any confirmation.

//   Always copies:
//     `cop a b` where b is an existing directory,
//     `cop` will ask for confirmation and then
//     will replace `b` with a copy of `a`, while `cp`
//     will copy `a` into the directory `b`

// Other improvements:

//   Existing location:
//     `cop a a` will return success code 0,
//     while `cp` will return error code 1.

//   Existing location with different path format:
//     `cop a /foo/a` where `a` is already located at `/foo/a`,
//     `cop` will note that `a` is already located at `/foo/a`
//     and return success code 0, while `cp` will consider this
//     a valid copy and `cp -i` will ask for confirmation.
// ";

// const DESTINATION_HELP: &str = const_format::formatcp!(
//     "The new path the copied files or directories should live at.
// To copy the files or directories into a directory, end the DESTINATION_PATH in {} or use the -i option",
//     std::path::MAIN_SEPARATOR
// );

// const USAGE: &str = "cop [OPTIONS] <SOURCE_PATHS>... <DESTINATION_PATH|--into <PATH>|--to <PATH>>";

// /// `cop`y files and directories
// #[derive(Parser, Debug)]
// #[command(author, version, about)]
// #[clap(after_long_help = DETAILS, override_usage = USAGE)]
// struct HelpCLI {
//     /// The paths of the files or directories to be copied
//     #[arg(required(true))]
//     source_paths: Vec<String>,

//     #[arg(help(DESTINATION_HELP))]
//     destination_path: String,

//     /// Copy files or directories into a directory at PATH.
//     #[arg(short, long, value_name = "PATH")]
//     into: Option<String>,

//     /// Copy and rename one file or directory from SOURCE_PATH to PATH.
//     #[arg(short, long, value_name = "PATH")]
//     to: Option<String>,

//     #[command(flatten)]
//     options: human_utils::StandardOptions,
// }

// // Because of limitations of clap, we use a different definition
// // to actually parse the arguments.
// // See https://github.com/clap-rs/clap/discussions/5774
// #[derive(Parser, Debug)]
// #[command(author, version, about)]
// #[clap(disable_help_flag = true)]
// struct CLI {
//     #[arg()]
//     paths: Vec<String>,

//     #[arg(short, long, conflicts_with("to"))]
//     into: Option<String>,

//     #[arg(short, long, conflicts_with("into"))]
//     to: Option<String>,

//     #[command(flatten)]
//     options: human_utils::StandardOptions,

//     #[arg(short, long)]
//     help: bool,
// }

fn main() {
    //     let mut help_command = HelpCLI::command();
    //     let args = &CLI::parse();
    //     let raw_args: Vec<String> = std::env::args().collect();
    //     let options = &args.options;

    //     if args.help {
    //         if raw_args.contains(&"-h".to_string()) {
    //             help_command.print_help().unwrap();
    //         } else {
    //             help_command.print_long_help().unwrap();
    //         }
    //         std::process::exit(SUCCESS);
    //     }

    //     let (paths, into, to) = determine_destination_type(args);
    //     at_least_one_source(&paths);
    //     let sources: &Vec<_> = &paths.iter().map(Utf8Path::new).collect();
    //     human_utils::set_color_override(&args.options);
    //     if let Some(destination) = into {
    //         let destination = Utf8Path::new(&destination);
    //         check_sources_exists(sources);
    //         check_sources_already_at_destination(options, sources, destination);
    //         let paths_at_destination = &get_paths_at_destination(sources, destination);
    //         human_utils::check_paths_exist_and_confirm_or_exit(options, paths_at_destination);
    //         let existing_ancestor = human_utils::find_existing_or_ancestor(options, destination);
    //         human_utils::create_directory(options, destination);
    //         copy_all(options, sources, paths_at_destination);
    //         print_success_all(options, sources, paths_at_destination, existing_ancestor);
    //     } else {
    //         let destination = to.unwrap();
    //         let destination = Utf8Path::new(&destination);
    //         let source = only_one_source(args, sources);
    //         let canonical_source = check_source_exists(source);
    //         check_source_already_at_destination(options, source, &canonical_source, destination);
    //         human_utils::check_path_exists_and_confirm_or_exit(options, destination);
    //         let existing_ancestor = human_utils::find_existing_ancestor_directory(options, destination);
    //         human_utils::create_parent_directory(options, destination);
    //         copy(options, source, destination);
    //         print_success(options, source, destination, existing_ancestor);
    //     }
    //     std::process::exit(SUCCESS);
}

// fn determine_destination_type(args: &CLI) -> (Vec<String>, Option<String>, Option<String>) {
//     let mut into = args.into.clone();
//     let mut to = args.to.clone();

//     if into.is_some() || to.is_some() {
//         return (args.paths.clone(), into, to);
//     }

//     // Split paths into sources and a destination
//     let mut paths = args.paths.clone();
//     let destination = paths.pop();

//     if let Some(destination) = destination {
//         if destination.ends_with(std::path::MAIN_SEPARATOR) {
//             into = Some(destination);
//         } else {
//             to = Some(destination);
//         }
//     } else {
//         eprintln!("Error: Expected either <DESTINATION_PATH> or --into PATH or --to PATH");
//         std::process::exit(FAILURE);
//     }

//     (paths, into, to)
// }

// fn at_least_one_source(paths: &Vec<String>) {
//     if paths.len() == 0 {
//         eprintln!("Error: Expected at least one SOURCE_PATH, got only a destination PATH");
//         std::process::exit(FAILURE);
//     }
// }

// fn only_one_source<'a>(args: &CLI, sources: &'a Vec<&'a Utf8Path>) -> &'a Utf8Path {
//     if sources.len() != 1 {
//         eprintln!(
//             "Error: Expected 1 SOURCE_PATH argument because {}, but got {}",
//             if args.into.is_some() {
//                 "the --into option was used".to_owned()
//             } else {
//                 format!(
//                     "DESTINATION_PATH did not end with a {}",
//                     std::path::MAIN_SEPARATOR
//                 )
//             },
//             sources.len()
//         );
//         std::process::exit(FAILURE);
//     }
//     sources[0]
// }

// fn check_sources_exists(sources: &Vec<&Utf8Path>) {
//     for source in sources {
//         check_source_exists(source);
//     }
// }

// fn check_source_exists(source: &Utf8Path) -> Utf8PathBuf {
//     match source.canonicalize_utf8() {
//         Ok(source) => source,
//         Err(error) => {
//             eprintln!("Error for \"{}\": {}", source, error);
//             std::process::exit(FAILURE);
//         }
//     }
// }

// fn check_sources_already_at_destination(
//     options: &StandardOptions,
//     sources: &Vec<&Utf8Path>,
//     destination: &Utf8Path,
// ) {
//     if let Ok(destination_canonical) = destination.canonicalize_utf8() {
//         let mut all_sources_already_at_destination = true;
//         for source in sources {
//             let source_canonical = source.canonicalize_utf8().unwrap();
//             let source_parent = source_canonical.parent().unwrap();
//             if source_parent.eq(&destination_canonical) {
//                 message_success!(
//                     options,
//                     "\"{}\" is already located at \"{}\"",
//                     source,
//                     human_utils::directory_path(destination)
//                 );
//             } else {
//                 all_sources_already_at_destination = false;
//             }
//         }
//         if all_sources_already_at_destination {
//             std::process::exit(SUCCESS);
//         }
//     }
// }

// fn check_source_already_at_destination(
//     options: &StandardOptions,
//     source: &Utf8Path,
//     canonical_source: &Utf8Path,
//     destination: &Utf8Path,
// ) {
//     if let Ok(canonical_destination) = destination.canonicalize_utf8() {
//         if canonical_source == canonical_destination {
//             message_success!(
//                 options,
//                 "\"{}\" is already located at \"{}\"",
//                 source,
//                 destination
//             );
//             std::process::exit(SUCCESS);
//         }
//     }
// }

// fn get_paths_at_destination(sources: &Vec<&Utf8Path>, destination: &Utf8Path) -> Vec<Utf8PathBuf> {
//     sources
//         .iter()
//         .map(|source| destination.join(source.file_name().unwrap()))
//         .collect()
// }

// fn copy_all(
//     options: &StandardOptions,
//     sources: &Vec<&Utf8Path>,
//     paths_at_destination: &Vec<Utf8PathBuf>,
// ) {
//     for (i, source) in sources.iter().enumerate() {
//         let destination_path = &paths_at_destination[i];
//         copy(options, source, &destination_path);
//     }
// }

// fn copy(options: &StandardOptions, from: &Utf8Path, to: &Utf8Path) {
//     if options.dry_run {
//         return;
//     }

//     if from.is_dir() {
//         if to.exists() {
//             if to.is_dir() {
//                 std::fs::remove_dir_all(to).unwrap();
//             } else {
//                 std::fs::remove_file(to).unwrap();
//             }
//         }
//         copy_dir_all(from, to).unwrap();
//     } else {
//         if to.exists() {
//             if to.is_dir() {
//                 std::fs::remove_dir_all(to).unwrap();
//             } else {
//                 std::fs::remove_file(to).unwrap();
//             }
//         }
//         std::fs::copy(from, to).unwrap();
//     }
// }

// fn copy_dir_all(from: &Utf8Path, to: &Utf8Path) -> std::io::Result<()> {
//     std::fs::create_dir_all(to)?;
//     for entry in std::fs::read_dir(from)? {
//         let entry = entry?;
//         let path = entry.path();
//         let target = to.join(path.file_name().unwrap());
//         if path.is_dir() {
//             copy_dir_all(&path, &target)?;
//         } else {
//             std::fs::copy(&path, &target)?;
//         }
//     }
//     Ok(())
// }

// const COLOR: colored::Color = colored::Color::BrightGreen;

// fn print_success_all(
//     options: &StandardOptions,
//     sources: &Vec<&Utf8Path>,
//     paths_at_destination: &Vec<Utf8PathBuf>,
//     existing_ancestor: Option<&Utf8Path>,
// ) {
//     for (i, source) in sources.iter().enumerate() {
//         let destination_path = &paths_at_destination[i];
//         print_success(options, source, destination_path, existing_ancestor);
//     }
// }

// fn print_success(
//     options: &StandardOptions,
//     source: &Utf8Path,
//     destination: &Utf8Path,
//     existing_ancestor: Option<&Utf8Path>,
// ) {
//     message_success!(
//         options,
//         "{} {} -> {}",
//         "C".color(COLOR),
//         path_string(source).bright_red(),
//         human_utils::color_new(destination, existing_ancestor, COLOR)
//     );
// }
