use anyhow::{ensure, Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, ren};

#[rstest]
fn silent_does_not_print_success_messages(#[values("-s", "--silent")] option: &str) -> Result<()> {
    let res = ren()
        .args(&["foo", "bar", option])
        .env(&env(&["foo"])?)
        .run()?;
    ensure!(res.output == "");
    Ok(())
}
