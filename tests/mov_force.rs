use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, mov, SUCCESS};

#[rstest]
fn force_does_not_ask_for_confirmation(#[values("-f", "--force")] option: &str) -> Result<()> {
    use colored::Colorize;

    let env = env(&["foo", "bar"])?;
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
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}
