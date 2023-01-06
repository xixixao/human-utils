use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, nam};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = nam().args(&["foo", "bar", option]).env(&env).run()?;
    ensure!(res.output == "\"foo\" -> \"bar\"");
    ensure!(env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}
