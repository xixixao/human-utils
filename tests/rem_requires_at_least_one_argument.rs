use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{rem, SUCCESS};

#[test]
fn no_args_fails() -> Result<()> {
    let code = rem().run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}
