use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_file() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "");
    Ok(())
}

#[test]
fn creates_directory() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a/"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a/".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("a"));
    Ok(())
}
