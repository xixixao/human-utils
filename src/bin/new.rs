use camino::Utf8Path;
use clap::Parser;
use colored::{ColoredString, Colorize};
use human_utils::{message_success, SUCCESS};

const SUCCESS_COLOR: colored::Color = colored::Color::BrightGreen;

const DETAILS: &str = "
Basic examples:

  Create a file with some text:
    `new path/to/new_file.txt -- Hello world`

  Create a directory:
    `new path/to/new_dir/`

  Create a directory with 3 empty files:
    `new path/to/new_dir/ a b c`

As part of `human-utils`, `new` asks for confirmation before
overwriting any file or directory.";

const PATH_HELP: &str = const_format::formatcp!(
    "The path including the name of the new file or directory.
To create a directory end the PATH in {} or use the -d option",
    std::path::MAIN_SEPARATOR
);

const FILE_HELP: &str = const_format::formatcp!(
    "Create a file, error if PATH ends in {}",
    std::path::MAIN_SEPARATOR
);

/// `new` - create a new file or directory
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(after_long_help = DETAILS)]
struct Args {
    #[arg(help(PATH_HELP))]
    path: String,

    /// Names of files to create in the new directory or in the same
    /// directory as the new file
    file_names: Vec<String>,

    /// The content of the new file(s).
    /// If not provided, the new file(s) will be empty
    // #[tested(nef_content)]
    #[arg(last(true))]
    content: Vec<String>,

    /// Create a directory.
    #[arg(short, long)]
    directory: bool,

    #[arg(long, help(FILE_HELP))]
    file: bool,

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

fn main() {
    let args = &Args::parse();
    let options = &args.options;
    let path = Utf8Path::new(&args.path);
    human_utils::set_color_override(options);
    if args.directory || args.path.ends_with(std::path::MAIN_SEPARATOR) {
        check_empty_directory_exists_already(args, path);
        check_path_exists(args, path);
        delete_directory_at_path(args, path);
        let existing_ancestor = human_utils::find_existing_ancestor_directory(options, path);
        create_directory(args, path);
        create_files(args, path);
        print_directory_success(args, path, existing_ancestor);
    } else {
        check_empty_file_exists_already(args, path);
        check_path_exists(args, path);
        delete_directory_at_path(args, path);
        let existing_ancestor = human_utils::find_existing_ancestor_directory(options, path);
        let parent_directory = human_utils::create_parent_directory(options, path);
        create_file(args, path);
        create_files(args, parent_directory);
        print_file_success(args, path, existing_ancestor);
    };
    std::process::exit(SUCCESS);
}

fn check_empty_directory_exists_already(args: &Args, path: &Utf8Path) {
    if !args.content.is_empty() {
        return;
    }

    if args.options.force {
        return;
    }

    if let Ok(metadata) = path.symlink_metadata() {
        if metadata.is_dir() && path.read_dir().unwrap().next().is_none() {
            message_success!(
                args,
                "{}",
                format!("Empty directory \"{}\" already exists", path).color(SUCCESS_COLOR)
            );
            std::process::exit(SUCCESS);
        }
    }
}

// TODO: #[tested(nef_empty_exists_already)]
fn check_empty_file_exists_already(args: &Args, path: &Utf8Path) {
    if !args.content.is_empty() {
        return;
    }

    if args.options.force {
        return;
    }

    if let Ok(metadata) = path.symlink_metadata() {
        if metadata.is_file() && metadata.len() == 0 {
            message_success!(
                args,
                "{}",
                format!("Empty file \"{}\" already exists", path).color(SUCCESS_COLOR)
            );
            std::process::exit(SUCCESS);
        }
    }
}

// TODO: #[tested(nef_replace_existing)]
fn check_path_exists(args: &Args, path: &Utf8Path) {
    if args.options.force {
        return;
    }

    human_utils::check_path_exists_and_confirm_or_exit(path);
}

fn delete_directory_at_path(args: &Args, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }

    if let Ok(metadata) = path.symlink_metadata() {
        if metadata.is_dir() {
            std::fs::remove_dir_all(path).unwrap();
        }
    }
}

fn create_directory(args: &Args, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }
    std::fs::create_dir_all(path).unwrap();
}

fn create_file(args: &Args, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }
    std::fs::write(path, args.content.join(" ")).unwrap();
}

fn create_files(args: &Args, parent_directory: &Utf8Path) {
    if args.options.dry_run {
        return;
    }

    for file_name in &args.file_names {
        let file_path = parent_directory.join(file_name);
        std::fs::write(file_path, "").unwrap();
    }
}

fn print_directory_success(args: &Args, path: &Utf8Path, existing_ancestor: Option<&Utf8Path>) {
    message_success!(
        args,
        "{}",
        join_colored(
            "N ".color(SUCCESS_COLOR),
            human_utils::color_new(
                &human_utils::directory_path(path),
                existing_ancestor,
                SUCCESS_COLOR
            )
        )
    );
}

// #[tested(nam_basic, nam_silent)]
fn print_file_success(args: &Args, file_path: &Utf8Path, existing_ancestor: Option<&Utf8Path>) {
    message_success!(
        args,
        "{}",
        join_colored(
            "N ".color(SUCCESS_COLOR),
            human_utils::color_new(file_path, existing_ancestor, SUCCESS_COLOR)
        )
    );
}

fn join_colored(first: ColoredString, second: ColoredString) -> ColoredString {
    if let Some(color) = first.fgcolor() {
        if Some(color) == second.fgcolor() {
            return format!("{}{}", first.clear(), second.clear()).color(color);
        }
    }
    format!("{}{}", first, second).normal()
}
