use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, mov, SUCCESS};

#[test]
fn same_exact_args() -> Result<()> {
    let res = mov().args(&["foo", "foo"]).env(&env(&["foo"])?).run()?;
    eq!(res.output,  "\"foo\" is already located at \"foo\"");
    ensure!(res.code == SUCCESS);
    Ok(())
}

#[test]
fn different_arg_same_canonical_path() -> Result<()> {
    let res = mov().args(&["foo", "./foo"]).env(&env(&["foo"])?).run()?;
    eq!(res.output,  "\"foo\" is already located at \"./foo\"");
    ensure!(res.code == SUCCESS);
    Ok(())
}
