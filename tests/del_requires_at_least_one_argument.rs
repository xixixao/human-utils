use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{del, SUCCESS};

#[test]
fn no_args_fails() -> Result<()> {
    let code = del().run()?.code;
    ensure!(code != SUCCESS);
    Ok(())
}
