use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::{FAILURE, SUCCESS};
use pretty_assertions::assert_eq;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_file_via_option() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["--file", "a"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "");
    Ok(())
}

#[test]
fn new_file_option_fails_with_directory_argument() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["--file", "a/"]).env(&env).run()?;
    assert_eq!(
        res.error,
        "Error: File path \"a/\" cannot end with a `/` when `--file` option is used."
    );
    ensure!(res.code == FAILURE);
    Ok(())
}
