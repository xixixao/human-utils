use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, nam, SUCCESS};

#[test]
fn same_exact_args() -> Result<()> {
    let res = nam().args(&["foo", "foo"]).env(&env(&["foo"])?).run()?;
    ensure!(res.output == "\"foo\" is already located at \"foo\"");
    ensure!(res.code == SUCCESS);
    Ok(())
}

#[test]
fn different_arg_same_canonical_path() -> Result<()> {
    let res = nam().args(&["foo", "./foo"]).env(&env(&["foo"])?).run()?;
    ensure!(res.output == "\"foo\" is already located at \"./foo\"");
    ensure!(res.code == SUCCESS);
    Ok(())
}
