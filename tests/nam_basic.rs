use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, nam, SUCCESS};

#[test]
fn renames_file() -> Result<()> {
    let env = env(&["foo"])?;
    let res = nam().args(&["foo", "bar"]).env(&env).run()?;
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}

#[test]
fn renames_directory() -> Result<()> {
    let env = env(&["foo/lorem"])?;
    let res = nam().args(&["foo", "bar"]).env(&env).run()?;
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar/lorem")? == "foo/lorem");
    Ok(())
}
