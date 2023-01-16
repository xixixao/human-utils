use std::io::Write;

use camino::Utf8Path;

pub fn set_color_override(options: &StandardOptions) {
    if options.color {
        colored::control::set_override(true);
    } else if options.no_color {
        colored::control::set_override(false);
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
