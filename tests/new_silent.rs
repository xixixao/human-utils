use anyhow::{ensure, Ok, Result};
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_file() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a", "--silent"]).env(&env).run()?;
    eq!(res.output, "");
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "");
    Ok(())
}

#[test]
fn creates_directory() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["a/", "--silent"]).env(&env).run()?;
    eq!(res.output, "");
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("a"));
    Ok(())
}
