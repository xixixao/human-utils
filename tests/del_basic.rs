use anyhow::{ensure, Ok, Result};
use colored::Colorize;

mod utils;

use crate::utils::{del, env, SUCCESS};

#[test]
fn removes_file() -> Result<()> {
    let env = env(&["foo"])?;
    let res = del().args(&["foo"]).answer("").env(&env).run()?;
    eq!(res.prompt, "Delete file \"foo\"? [Y/n]");
    eq!(res.output, "D foo".bright_red().to_string());
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    Ok(())
}

#[test]
fn removes_directory() -> Result<()> {
    let env = env(&["foo/lorem"])?;
    let res = del().args(&["foo"]).answer("").env(&env).run()?;
    eq!(res.prompt, "Delete directory \"foo\"? [Y/n]");
    eq!(res.output, "D foo/".bright_red().to_string());
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    Ok(())
}
