use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_directory_via_option() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["--directory", "a"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "a/".bright_green())
    );
    ensure!(res.code == SUCCESS);
    ensure!(env.exists_directory("a"));
    Ok(())
}
