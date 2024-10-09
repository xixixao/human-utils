use anyhow::{Ok, Result};
use rstest::rstest;

mod utils;

use crate::utils::{env, mov};

#[rstest]
fn silent_does_not_print_success_messages(#[values("-s", "--silent")] option: &str) -> Result<()> {
    let res = mov()
        .args(&["foo", "bar", option])
        .env(&env(&["foo"])?)
        .run()?;
    eq!(res.output, "");
    Ok(())
}
