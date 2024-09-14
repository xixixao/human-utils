use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{del, env};

#[rstest]
fn silent_does_not_print_success_messages(#[values("-s", "--silent")] option: &str) -> Result<()> {
    let env = env(&["foo"])?;
    let res = del().args(&["foo", option]).answer("").env(&env).run()?;
    ensure!(res.output == "");
    Ok(())
}
