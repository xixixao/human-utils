use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{mov, SUCCESS};

#[test]
fn no_args_fails() -> Result<()> {
    let code = mov().run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}

#[test]
fn one_arg_fails() -> Result<()> {
    let code = mov().args(&["foo"]).run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}
