use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, mov, SUCCESS};
use colored::Colorize;

#[test]
fn renames_file() -> Result<()> {
    let env = env(&["foo"])?;
    let res = mov().args(&["foo", "bar"]).env(&env).run()?;
    eq!(
        res.output,
        format!(
            "{} {} -> {}",
            "M".bright_green(),
            "foo".bright_red(),
            "bar".bright_green()
        )
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}

#[test]
fn renames_directory() -> Result<()> {
    let env = env(&["foo/lorem"])?;
    let res = mov().args(&["foo", "bar"]).env(&env).run()?;
    eq!(
        res.output,
        format!(
            "{} {} -> {}",
            "M".bright_green(),
            "foo".bright_red(),
            "bar".bright_green()
        )
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar/lorem")? == "foo/lorem");
    Ok(())
}

#[test]
fn moves_into_directory() -> Result<()> {
    let env = env(&["foo/lorem", "bar"])?;
    let res = mov().args(&["bar", "foo/"]).env(&env).run()?;
    eq!(
        res.output,
        format!(
            "{} {} -> {}{}",
            "M".bright_green(),
            "bar".bright_red(),
            "foo/",
            "bar".bright_green()
        )
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("bar"));
    ensure!(env.read("foo/bar")? == "bar");
    Ok(())
}
