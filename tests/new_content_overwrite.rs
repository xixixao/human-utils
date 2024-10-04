use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn overwrites_empty_file_with_content() -> Result<()> {
    let env = env(&["a"])?;
    env.write("a", "")?;
    let res = new().args(&["a", "--", "Hello world"]).env(&env).run()?;
    eq!(
        res.output,
        format!("{} {}", "M".bright_green(), "a".bright_green(),)
    );
    eq!(env.read("a")?, "Hello world\n");
    Ok(())
}

#[test]
fn prompts_to_overwrite_file_with_content() -> Result<()> {
    let env = env(&["a"])?;
    env.write("a", "So important!")?;
    let res = new()
        .args(&["a", "--", "Hello world"])
        .answer("y")
        .env(&env)
        .run()?;
    eq!(res.prompt, "Overwrite file \"a\"? [Y/n]");
    eq!(
        res.output,
        format!("{} {}", "M".bright_green(), "a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "Hello world\n");
    Ok(())
}

#[test]
fn prompts_to_erase_file() -> Result<()> {
    let env = env(&["a"])?;
    env.write("a", "So important!")?;
    let res = new().args(&["a", "--", ""]).answer("y").env(&env).run()?;
    eq!(res.prompt, "Overwrite file \"a\"? [Y/n]");
    eq!(
        res.output,
        format!("{} {}", "M".bright_green(), "a".bright_green(),)
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "");
    Ok(())
}
