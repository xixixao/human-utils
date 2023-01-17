use camino::Utf8Path;
use clap::Parser;
use colored::{ColoredString, Colorize};
use human_utils::{message_success, SUCCESS};

/// `nef` - create a new file
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CLI {
    /// The path including the name the file should live at
    // #[tested(nef_requires_one_argument)]
    file_path: String,

    /// The content of the file
    /// If not provided, the file will be empty
    // #[tested(nef_content)]
    content: Option<String>,

    #[command(flatten)]
    options: human_utils::StandardOptions,
}

fn main() {
    let args = &CLI::parse();
    let options = &args.options;
    let path = Utf8Path::new(&args.file_path);
    human_utils::set_color_override(options);
    check_empty_file_exists_already(args, path);
    check_path_exists(args, path);
    delete_directory_at_path(args, path);
    let existing_ancestor = human_utils::find_existing_ancestor_directory(options, path);
    human_utils::create_parent_directory(options, path);
    create_file(args, path);
    print_success(args, path, existing_ancestor);
    std::process::exit(SUCCESS);
}

// TODO: #[tested(nef_empty_exists_already)]
fn check_empty_file_exists_already(args: &CLI, path: &Utf8Path) {
    if args.content.is_some() {
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
                format!("Empty file \"{}\" already exists", path).color(COLOR)
            );
            std::process::exit(SUCCESS);
        }
    }
}

// TODO: #[tested(nef_replace_existing)]
fn check_path_exists(args: &CLI, path: &Utf8Path) {
    if args.options.force {
        return;
    }

    human_utils::check_path_exists_and_confirm_or_exit(path);
}

fn delete_directory_at_path(args: &CLI, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }

    if let Ok(metadata) = path.symlink_metadata() {
        if metadata.is_dir() {
            std::fs::remove_dir_all(path).unwrap();
        }
    }
}

fn create_file(args: &CLI, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }
    std::fs::write(path, args.content.as_deref().unwrap_or("")).unwrap();
}

const COLOR: colored::Color = colored::Color::BrightGreen;

// #[tested(nam_basic, nam_silent)]
fn print_success(args: &CLI, file_path: &Utf8Path, existing_ancestor: Option<&Utf8Path>) {
    message_success!(
        args,
        "{}",
        join_colored(
            "N ".color(COLOR),
            human_utils::color_new(file_path, existing_ancestor, COLOR)
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
