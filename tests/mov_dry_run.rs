use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, mov};

#[rstest]
fn dry_run_doesnt_perform_changes(#[values("-n", "--dry-run")] option: &str) -> Result<()> {
    use colored::Colorize;

    let env = env(&["foo"])?;
    let res = mov().args(&["foo", "bar", option]).env(&env).run()?;
    eq!(
        res.output,
        format!(
            "{} {} -> {}",
            "M".bright_green(),
            "foo".bright_red(),
            "bar".bright_green()
        )
    );

    ensure!(env.exists("foo"));
    ensure!(!env.exists("bar"));
    Ok(())
}
