mod lazy_path;

pub use lazy_path::LazyPath;

use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use colored::{ColoredString, Colorize};

pub fn set_color_override(options: &StandardOptions) {
    if options.color {
        colored::control::set_override(true);
    } else if options.no_color {
        colored::control::set_override(false);
    }
}

pub fn find_existing_or_ancestor<'a>(
    options: &StandardOptions,
    path: &'a Utf8Path,
) -> Option<&'a Utf8Path> {
    if options.no_color {
        return None;
    }

    if path.exists() {
        return Some(path);
    }

    find_existing_ancestor_directory(options, path)
}

pub fn find_existing_ancestor_directory<'a>(
    options: &StandardOptions,
    path: &'a Utf8Path,
) -> Option<&'a Utf8Path> {
    if options.no_color {
        return None;
    }

    let mut ancestor = path.parent();
    while let Some(ancestor_path) = ancestor {
        if ancestor_path.exists() {
            return Some(ancestor_path);
        }
        ancestor = ancestor_path.parent();
    }
    None
}

pub fn create_directory(options: &StandardOptions, path: &Utf8Path) {
    if options.dry_run {
        return;
    }
    std::fs::create_dir_all(path).unwrap();
}

pub fn create_file(options: &StandardOptions, path: &Utf8Path, content: Option<&str>) {
    if options.dry_run {
        return;
    }
    create_parent_directory(options, path);
    if let Some(text) = content {
        std::fs::write(path, text).unwrap();
    } else {
        std::fs::File::create(path).unwrap();
    }
}

pub fn create_parent_directory<'a>(options: &StandardOptions, path: &'a Utf8Path) -> &'a Utf8Path {
    let parent_directory = path.parent().unwrap();
    if !options.dry_run {
        std::fs::create_dir_all(parent_directory).unwrap();
    }
    parent_directory
}

pub fn check_path_exists_and_confirm_or_exit(options: &StandardOptions, path: &Utf8Path) {
    if options.force {
        return;
    }

    if let Ok(metadata) = path.symlink_metadata() {
        let file_type = if metadata.is_dir() {
            "Directory"
        } else {
            "File"
        };
        print!(
            "{} \"{}\" already exists, replace it? [Y/n]",
            file_type, path
        );
        confirm_or_exit();
    }
}

pub fn check_paths_exist_and_confirm_or_exit(options: &StandardOptions, paths: &Vec<Utf8PathBuf>) {
    if options.force {
        return;
    }
    let clashing: Vec<_> = paths.iter().filter(|path| path.exists()).collect();
    ask_to_overwrite(&clashing);
}

pub fn ask_to_overwrite<P: AsRef<Utf8Path>>(clashing: &Vec<P>) {
    if clashing.is_empty() {
        return;
    }
    if let [path] = clashing.as_slice() {
        ask_for_single_path(path.as_ref());
    } else {
        ask_for_multiple_paths(&clashing);
    }
    confirm_or_exit();
}

pub fn confirm_or_exit() {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input != "\n" && !input.trim().to_lowercase().starts_with('y') {
        std::process::exit(FAILURE);
    }
}

fn ask_for_single_path(path: &Utf8Path) {
    let metadata = path.symlink_metadata().unwrap();
    let file_type = if metadata.is_dir() {
        "directory"
    } else {
        "file"
    };
    print!("Overwrite {} \"{}\"? [Y/n]", file_type, path);
}

pub fn ask_for_multiple_paths<P: AsRef<Utf8Path>>(paths: &Vec<P>) {
    println!("For the following...");
    for path in paths {
        let metadata = path.as_ref().symlink_metadata().unwrap();
        print_path(path.as_ref(), &metadata);
    }
    print!("...overwrite all? [Y/n]");
}

fn print_path(path: &Utf8Path, metadata: &std::fs::Metadata) {
    println!("{}", format_path(path, metadata));
}

pub fn format_path(path: &Utf8Path, metadata: &std::fs::Metadata) -> String {
    if metadata.is_dir() {
        directory_path(path).to_string()
    } else {
        path_string(path)
    }
}

pub fn color_new(
    path: &Utf8Path,
    existing_ancestor: Option<&Utf8Path>,
    color: colored::Color,
) -> ColoredString {
    quote_spaced(
        path,
        if let Some(existing_ancestor_path) = existing_ancestor {
            format!(
                "{}{}{}",
                existing_ancestor_path
                    .to_string()
                    .trim_end_matches(std::path::MAIN_SEPARATOR),
                std::path::MAIN_SEPARATOR,
                strip_path_prefix(path, existing_ancestor_path).color(color)
            )
            .normal()
        } else {
            path_string(path).color(color)
        },
        color,
    )
}

fn strip_path_prefix(path: &Utf8Path, prefix: &Utf8Path) -> String {
    let new_path = path.strip_prefix(prefix).unwrap();
    if path.as_str().ends_with(std::path::MAIN_SEPARATOR) {
        directory_path(new_path).to_string()
    } else {
        new_path.to_string()
    }
}

fn quote_spaced(
    path: &Utf8Path,
    colored_path: ColoredString,
    color: colored::Color,
) -> ColoredString {
    let path_str: &str = path.as_ref();
    if path_str.contains(' ') {
        let quote = "\"".color(color);
        format!("{}{}{}", quote, colored_path, quote).normal()
    } else {
        colored_path
    }
}

pub fn directory_path(path: &Utf8Path) -> Utf8PathBuf {
    if path.as_str().ends_with(std::path::MAIN_SEPARATOR) {
        return path.to_owned();
    }
    Utf8PathBuf::from(format!("{}{}", path, std::path::MAIN_SEPARATOR))
}

pub fn path_string<S: AsRef<str>>(path: S) -> String
where
    S: AsRef<str>,
{
    let path = path.as_ref();
    if path.contains(' ') {
        format!("\"{}\"", path)
    } else {
        path.to_owned()
    }
}

#[macro_export]
macro_rules! message_success {
    ($options:ident, $($arg:tt)*) => {
        if !$options.silent {
            println!($($arg)*);
        }
    };
}

pub const SUCCESS: i32 = 0;
pub const FAILURE: i32 = 1;

#[derive(clap::Args, Debug, Clone)]
pub struct StandardOptions {
    /// Never ask for confirmation
    #[arg(short, long)]
    // #[tested(nam_force)]
    pub force: bool,

    /// Do not print success messages, still print errors
    #[arg(short, long)]
    // #[tested(nam_silent)]
    pub silent: bool,

    /// Ask for confirmation, print success messages and errors, but do not perform any changes
    #[arg(short = 'n', long)]
    // #[tested(nam_dry_run)]
    pub dry_run: bool,

    /// Always color output
    #[arg(long)]
    pub color: bool,

    /// Never color output
    #[arg(long, conflicts_with = "color")]
    pub no_color: bool,
}

// a unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_path_prefix() {
        let path = Utf8PathBuf::from("/a/b/c");
        let prefix = Utf8PathBuf::from("/a");
        assert_eq!(strip_path_prefix(&path, &prefix), "b/c");
    }
}
