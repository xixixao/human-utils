use camino::Utf8Path;
use clap::Parser;
use colored::Colorize;
use human_utils::{message_success, path_string, SUCCESS};

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
    let args = CLI::parse();
    let path = Utf8Path::new(&args.file_path);
    human_utils::set_color_override(&args.options);
    check_empty_file_exists_already(&args, &path);
    check_path_exists(&args, &path);
    delete_directory_at_path(&args, &path);
    create_directory(&args, &path);
    create_file(&args, &path);
    print_success(&args);
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
            print!("Empty file \"{}\" already exists", path);
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

fn create_directory(args: &CLI, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }
    if let Some(directory) = path.parent() {
        std::fs::create_dir_all(directory).unwrap();
    }
}

fn create_file(args: &CLI, path: &Utf8Path) {
    if args.options.dry_run {
        return;
    }
    std::fs::write(path, args.content.as_deref().unwrap_or("")).unwrap();
}

// #[tested(nam_basic, nam_silent)]
fn print_success(args: &CLI) {
    message_success!(
        args,
        "{}",
        format!("N {}", path_string(&args.file_path)).bright_green()
    );
}
