use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, ren, SUCCESS};
use colored::Colorize;

#[test]
fn renames_file() -> Result<()> {
    let env = env(&["foo"])?;
    let res = ren().args(&["foo", "bar"]).env(&env).run()?;
    ensure!(
        res.output
            == format!(
                "{} {} -> {}",
                "R".bright_green(),
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
    let res = ren().args(&["foo", "bar"]).env(&env).run()?;
    ensure!(
        res.output
            == format!(
                "{} {} -> {}",
                "R".bright_green(),
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
