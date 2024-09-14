use anyhow::{ensure, Ok, Result};
use regex::Regex;

pub mod utils;
use crate::utils::{del, env, SUCCESS};

#[test]
fn nonexistent_path_fails() -> Result<()> {
    let res = del().args(&["foo"]).run()?;
    ensure!(res.error.starts_with("Error for \"foo\":"));
    ensure!(res.code != SUCCESS);
    Ok(())
}

#[test]
fn nonexistent_paths_fail() -> Result<()> {
    let res = del().args(&["foo", "bar"]).run()?;
    let error_pattern = Regex::new(
        "^\"foo\" error:.*\n\"bar\" error:.*\n...no files or directories can be removed.$",
    )
    .unwrap();
    ensure!(error_pattern.is_match(&res.error));
    ensure!(res.code != SUCCESS);
    Ok(())
}

#[test]
fn mix_of_existing_and_not_succeeds() -> Result<()> {
    let env = env(&["foo"])?;
    let res = del().args(&["foo", "bar"]).answer("").env(&env).run()?;
    ensure!(res.prompt == "For the following...\nfoo\n...delete all existing? [Y/n]");
    ensure!(res.error.starts_with("\"bar\" error:"));
    ensure!(res.output == "D foo");
    ensure!(res.code == SUCCESS);
    Ok(())
}
