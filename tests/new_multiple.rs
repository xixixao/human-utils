use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_files_and_directories() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a/", "b", "c", "d/"]).env(&env).run()?;
    eq!(
        res.output,
        format!(
            "{}\n{}\n{}\n{}",
            format!("{} {}", "N".bright_green(), "a/".bright_green()),
            format!("{} {}", "N".bright_green(), "d/".bright_green()),
            format!("{} {}", "N".bright_green(), "b".bright_green()),
            format!("{} {}", "N".bright_green(), "c".bright_green())
        )
    );
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("a"));
    ensure!(env.exists_directory("d"));
    eq!(env.read("b")?, "");
    eq!(env.read("c")?, "");
    Ok(())
}

#[test]
fn creates_nested_directories() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a/b/", "a/", "a/b/c/"]).env(&env).run()?;
    // It doesn't feel worth it to fix the fact that we print out the parents
    // if they were requested.
    eq!(
        res.output,
        format!(
            "{}\n{}\n{}",
            format!("{} {}", "N".bright_green(), "a/".bright_green()),
            format!("{} {}", "N".bright_green(), "a/b/".bright_green()),
            format!("{} {}", "N".bright_green(), "a/b/c/".bright_green())
        )
    );
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("a/b/c"));
    Ok(())
}
