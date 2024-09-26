use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn creates_file_with_content() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["foo", "--", "Hello world"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "N".bright_green(), "foo".bright_green())
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("foo")?, "Hello world\n");
    Ok(())
}

#[test]
fn creates_files_with_content() -> Result<()> {
    let env = env(&[])?;
    let res = new()
        .args(&["a/", "b", "c", "--", "Hello world"])
        .env(&env)
        .run()?;
    eq!(
        res.output,
        format!(
            "{}\n{}\n{}",
            format!("{} {}", "N".bright_green(), "a/".bright_green()),
            format!("{} {}", "N".bright_green(), "b".bright_green()),
            format!("{} {}", "N".bright_green(), "c".bright_green())
        )
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("b")?, "Hello world\n");
    eq!(env.read("c")?, "Hello world\n");
    Ok(())
}
