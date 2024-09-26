use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{del, env, SUCCESS};

#[rstest]
fn force_does_not_ask_for_confirmation(#[values("-f", "--force")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = del().args(&["foo", option]).env(&env).run()?;
    eq!(res.output,  "D foo");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    Ok(())
}
