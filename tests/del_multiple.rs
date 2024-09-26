use anyhow::{ensure, Ok, Result};
use colored::Colorize;

mod utils;

use crate::utils::{del, env, SUCCESS};

#[test]
fn removes_files() -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = del()
        .args(&["foo", "bar", "--color"])
        .answer("")
        .env(&env)
        .run()?;
    eq!(
        res.prompt,
        "For the following...\nfoo\nbar\n...delete all? [Y/n]"
    );
    println!("{:?}", res.output);
    eq!(
        res.output,
        format!("{}\n{}", "D foo".bright_red(), "D bar".bright_red())
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}

#[test]
fn removes_directories() -> Result<()> {
    let env = env(&["foo/lorem", "bar/ipsum"])?;
    let res = del()
        .args(&["foo", "bar", "--color"])
        .answer("")
        .env(&env)
        .run()?;
    eq!(
        res.prompt,
        "For the following...\nfoo/\nbar/\n...delete all? [Y/n]"
    );
    eq!(
        res.output,
        format!("{}\n{}", "D foo/".bright_red(), "D bar/".bright_red())
    );
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo/lorem"));
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("bar/lorem"));
    ensure!(!env.exists("bar"));
    Ok(())
}
