use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, rem, SUCCESS};

#[rstest]
fn force_does_not_ask_for_confirmation(#[values("-f", "--force")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = rem().args(&["foo", option]).env(&env).run()?;
    ensure!(res.output == "Done");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    Ok(())
}
