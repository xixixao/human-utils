use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, rem, SUCCESS};

#[test]
fn removes_file() -> Result<()> {
    let env = env(&["foo"])?;
    let res = rem().args(&["foo"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "Remove file \"foo\"? [Y/n]");
    ensure!(res.output == "Done");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    Ok(())
}

#[test]
fn removes_directory() -> Result<()> {
    let env = env(&["foo/lorem"])?;
    let res = rem().args(&["foo"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "Remove directory \"foo\"? [Y/n]");
    ensure!(res.output == "Done");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    Ok(())
}
