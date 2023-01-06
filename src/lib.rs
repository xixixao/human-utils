use std::io::Write;

use colored::*;

pub fn quote(path: &str) -> String {
    let quote = "\"".bright_black();
    format!("{}{}{}", quote, path, quote)
}

pub fn confirm_or_exit() {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input != "\n" && !input.trim().to_lowercase().starts_with('y') {
        std::process::exit(FAILURE);
    }
}

#[macro_export]
macro_rules! message_success {
    ($command_arg:ident, $($arg:tt)*) => {
        if !$command_arg.silent {
            println!($($arg)*);
        }
    };
}

pub const SUCCESS: i32 = 0;
pub const FAILURE: i32 = 1;
