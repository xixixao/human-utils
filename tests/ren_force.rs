use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, ren, SUCCESS};

#[rstest]
fn force_does_not_ask_for_confirmation(#[values("-f", "--force")] option: &str) -> Result<()> {
    let env = env(&["foo", "bar"])?;
    let res = ren().args(&["foo", "bar", option]).env(&env).run()?;
    eq!(res.output, "R foo -> bar");
    ensure!(res.code == SUCCESS);
    ensure!(!env.exists("foo"));
    ensure!(env.read("bar")? == "foo");
    Ok(())
}
