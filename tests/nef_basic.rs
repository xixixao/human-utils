use anyhow::{ensure, Ok, Result};
use colored::Colorize;

mod utils;

use crate::utils::{env, nef, SUCCESS};

#[test]
fn creates_a_file() -> Result<()> {
    let env = env(&[])?;
    let res = nef().args(&["foo"]).env(&env).run()?;
    ensure!(res.output == "N foo".bright_green().to_string());
    ensure!(res.code == SUCCESS);
    ensure!(env.read("foo")? == "");
    Ok(())
}
