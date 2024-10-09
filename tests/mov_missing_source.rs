use anyhow::{ensure, Ok, Result};

pub mod utils;

use crate::utils::{mov, SUCCESS};

#[test]
fn nonexistent_source_fails() -> Result<()> {
    let res = mov().args(&["foo", "bar"]).run()?;
    ensure!(res.error.starts_with("Error for \"foo\":"));
    ensure!(res.code != SUCCESS);
    Ok(())
}
