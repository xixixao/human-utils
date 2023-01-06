use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, rem, SUCCESS};

#[test]
fn removes_files() -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = rem().args(&["foo", "bar"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "For the following...\n\"foo\"\n\"bar\"\n...remove all? [Y/n]");
    ensure!(res.output == "Done");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}

#[test]
fn removes_directories() -> Result<()> {
    let env = env(&["foo/lorem", "bar/ipsum"])?;
    let res = rem().args(&["foo", "bar"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "For the following...\n\"foo/\"\n\"bar/\"\n...remove all? [Y/n]");
    ensure!(res.output == "Done");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("bar/lorem"));
    ensure!(!env.exists("bar"));
    Ok(())
}
