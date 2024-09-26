use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{del, env};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = del().args(&["foo", option]).answer("").env(&env).run()?;
    eq!(res.output,  "D foo");
    ensure!(env.exists("foo"));
    Ok(())
}
