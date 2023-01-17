use std::io::Write;

use camino::Utf8Path;
use colored::{ColoredString, Colorize};

pub fn set_color_override(options: &StandardOptions) {
    if options.color {
        colored::control::set_override(true);
    } else if options.no_color {
        colored::control::set_override(false);
    }
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

pub fn create_parent_directory(options: &StandardOptions, path: &Utf8Path) {
    if options.dry_run {
        return;
    }
    if let Some(directory) = path.parent() {
        std::fs::create_dir_all(directory).unwrap();
    }
}

pub fn check_path_exists_and_confirm_or_exit(path: &Utf8Path) {
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

pub fn confirm_or_exit() {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input != "\n" && !input.trim().to_lowercase().starts_with('y') {
        std::process::exit(FAILURE);
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
                existing_ancestor_path,
                std::path::MAIN_SEPARATOR,
                path.strip_prefix(existing_ancestor_path)
                    .unwrap()
                    .to_string()
                    .color(color)
            )
            .normal()
        } else {
            path_string(path).color(color)
        },
        color,
    )
}

fn quote_spaced(
    path: &Utf8Path,
    colored_path: ColoredString,
    color: colored::Color,
) -> ColoredString {
    let path_str: &str = path.as_ref();
    if path_str.contains(" ") {
        let quote = "\"".color(color);
        format!("{}{}{}", quote, colored_path, quote).normal()
    } else {
        colored_path
    }
}

pub fn directory_string(path: &Utf8Path) -> String {
    if path.as_str().ends_with(std::path::MAIN_SEPARATOR) {
        return path.to_string();
    }
    format!("{}{}", path, std::path::MAIN_SEPARATOR)
}

pub fn path_string<S: AsRef<str>>(path: S) -> String
where
    S: AsRef<str>,
{
    let path = path.as_ref();
    if path.contains(" ") {
        format!("\"{}\"", path)
    } else {
        path.to_owned()
    }
}

#[macro_export]
macro_rules! message_success {
    ($command_arg:ident, $($arg:tt)*) => {
        if !$command_arg.options.silent {
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

    /// Do not print success messages, still prints errors
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
