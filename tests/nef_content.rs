use anyhow::{ensure, Ok, Result};

mod utils;

use crate::utils::{env, new};

#[test]
fn writes_content() -> Result<()> {
    let env = env(&[])?;
    new().args(&["foo", "Hello world"]).env(&env).run()?;
    ensure!(env.read("foo")? == "Hello world");
    Ok(())
}
