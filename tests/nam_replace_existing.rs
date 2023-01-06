use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, nam, SUCCESS};

#[test]
fn asks_for_confirmation() -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = nam().args(&["foo", "bar"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "File \"bar\" already exists, replace it? [Y/n]");
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}

#[rstest]
fn valid_confirmations(#[values("y", "Y", "yes")] value: &str) -> Result<()> {
    let res = nam()
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
    let res = nam().args(&["foo", "bar"]).answer("n").env(&env).run()?;
    ensure!(res.prompt == "File \"bar\" already exists, replace it? [Y/n]");
    ensure!(res.output == "");
    ensure!(res.code != SUCCESS);
    ensure!(env.exists("foo"));
    ensure!(env.read("bar")? == "bar");
    Ok(())
}

#[rstest]
fn valid_rejections(#[values("n", "N", "no", "boo")] value: &str) -> Result<()> {
    let res = nam()
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
    let res = nam().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    ensure!(res.prompt == "Directory \"bar\" already exists, replace it? [Y/n]");
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("bar/baz"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}

#[test]
fn dir_replacing_file() -> Result<()> {
    let env = env(&["foo/baz", "bar"])?;
    let res = nam().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    ensure!(res.prompt == "File \"bar\" already exists, replace it? [Y/n]");
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("foo/baz"));
    ensure!(env.exists("bar/baz"));
    ensure!(env.read("bar/baz")? == "foo/baz");
    Ok(())
}

#[test]
fn dir_replacing_dir() -> Result<()> {
    let env = env(&["foo/baz", "bar/lorem"])?;
    let res = nam().args(&["foo", "bar"]).answer("\n").env(&env).run()?;
    ensure!(res.prompt == "Directory \"bar\" already exists, replace it? [Y/n]");
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(!env.exists("foo/baz"));
    ensure!(!env.exists("foo/lorem"));
    ensure!(env.exists("bar/baz"));
    ensure!(env.read("bar/baz")? == "foo/baz");
    Ok(())
}
