use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn dry_run_does_not_create_file() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a", "--dry-run"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("a"));
    Ok(())
}

#[test]
fn dry_run_does_not_create_directory() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a/", "--dry-run"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a/".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists_directory("a"));
    Ok(())
}
