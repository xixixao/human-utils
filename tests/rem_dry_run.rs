use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, rem};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = rem().args(&["foo", option]).answer("").env(&env).run()?;
    ensure!(res.output == "Done");
    ensure!(env.exists("foo"));
    Ok(())
}
