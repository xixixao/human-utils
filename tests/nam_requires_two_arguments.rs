use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{ren, SUCCESS};

#[test]
fn no_args_fails() -> Result<()> {
    let code = ren().run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}

#[test]
fn one_arg_fails() -> Result<()> {
    let code = ren().args(&["foo"]).run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}
