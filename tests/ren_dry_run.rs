use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, ren};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = ren().args(&["foo", "bar", option]).env(&env).run()?;
    eq!(res.output,  "R foo -> bar");
    ensure!(env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}
