use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;
use colored::Colorize;

use crate::utils::{env, ren};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = ren().args(&["foo", "bar", option]).env(&env).run()?;
    ensure!(res.output == "M foo -> bar".bright_blue().to_string());
    ensure!(env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}
