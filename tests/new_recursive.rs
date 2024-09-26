use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_directories_for_file() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["c/b/a"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "c/b/a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("c/b/a")?, "");
    Ok(())
}

#[test]
fn creates_directories_for_directory() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["c/b/a/"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "c/b/a/".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("c/b/a"));
    Ok(())
}
