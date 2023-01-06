use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{nam, SUCCESS};

#[test]
fn no_args_fails() -> Result<()> {
    let code = nam().run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}

#[test]
fn one_arg_fails() -> Result<()> {
    let code = nam().args(&["foo"]).run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}
