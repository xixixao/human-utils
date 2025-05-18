use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use rstest::rstest;

mod utils;

use crate::utils::{env, mov, SUCCESS};

#[test]
fn asks_for_confirmation() -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = mov().args(&["foo", "bar"]).answer("").env(&env).run()?;
    eq!(res.prompt, "File \"bar\" already exists, replace it? [Y/n]");
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

#[rstest]
fn valid_confirmations(#[values("y", "Y", "yes")] value: &str) -> Result<()> {
    let res = mov()
        .args(&["foo", "bar"])
        .answer(value)
        .env(&env(&["foo", "bar"])?)
        .run()?;
    ensure!(res.code == SUCCESS);
    Ok(())
}

#[test]
fn without_confirmation_does_nothing() -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = mov().args(&["foo", "bar"]).answer("n").env(&env).run()?;
    eq!(res.prompt, "File \"bar\" already exists, replace it? [Y/n]");
    eq!(res.output, "");
    ensure!(res.code != SUCCESS);
    ensure!(env.exists("foo"));
    ensure!(env.read("bar")? == "bar");
    Ok(())
}

#[rstest]
fn valid_rejections(#[values("n", "N", "no", "boo")] value: &str) -> Result<()> {
    let res = mov()
        .args(&["foo", "bar"])
        .answer(value)
        .env(&env(&["foo", "bar"])?)
        .run()?;
    ensure!(res.code != SUCCESS);
    Ok(())
}

#[test]
fn file_replacing_dir() -> Result<()> {
    let env = env(&["foo", "bar/baz"])?;
    let res = mov().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    eq!(
        res.prompt,
        "Directory \"bar\" already exists, replace it? [Y/n]"
    );
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
    Ok(())
}

#[test]
fn dir_replacing_file() -> Result<()> {
    let env = env(&["foo/baz", "bar"])?;
    let res = mov().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    eq!(res.prompt, "File \"bar\" already exists, replace it? [Y/n]");
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
    Ok(())
}

#[test]
fn dir_replacing_dir() -> Result<()> {
    let env = env(&["foo/baz", "bar/lorem"])?;
    let res = mov().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    eq!(
        res.prompt,
        "Directory \"bar\" already exists, replace it? [Y/n]"
    );
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
    ensure!(!env.exists("foo/baz"));
    ensure!(!env.exists("bar/lorem"));
    Ok(())
}
