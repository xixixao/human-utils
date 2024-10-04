use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::SUCCESS;

mod utils;

use crate::utils::{env, new};

#[test]
fn prompts_to_erase_directory() -> Result<()> {
    let env = env(&["a/b"])?;
    let res = new().args(&["a"]).answer("y").env(&env).run()?;
    eq!(res.prompt, "Overwrite directory \"a\"? [Y/n]");
    eq!(
        res.output,
        format!(
            "{}\n{}",
            format!("{}", "D a/".bright_red(),),
            format!("{} {}", "N".bright_green(), "a".bright_green(),)
        )
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a")?, "");
    Ok(())
}

#[test]
fn prompts_to_erase_nested_directory() -> Result<()> {
    let env = env(&["a/b/c"])?;
    let res = new().args(&["a/b"]).answer("y").env(&env).run()?;
    eq!(res.prompt, "Overwrite directory \"a/b\"? [Y/n]");
    eq!(
        res.output,
        format!(
            "{}\n{}",
            format!("{}", "D a/b/".bright_red(),),
            format!("{} {}{}", "N".bright_green(), "a/", "b".bright_green(),)
        )
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a/b")?, "");
    Ok(())
}

#[test]
fn prompts_to_erase_nested_file() -> Result<()> {
    let env = env(&["a/b"])?;
    let res = new().args(&["a/b/c"]).answer("y").env(&env).run()?;
    eq!(res.prompt, "Overwrite file \"a/b\"? [Y/n]");
    eq!(
        res.output,
        format!(
            "{}\n{}",
            format!("{}", "D a/b".bright_red(),),
            format!("{} {}{}", "N".bright_green(), "a/", "b/c".bright_green(),)
        )
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a/b/c")?, "");
    Ok(())
}

#[test]
fn noops_on_directory_that_already_exists() -> Result<()> {
    let env = env(&["a/b"])?;
    let res = new().args(&["a/"]).env(&env).run()?;
    eq!(res.output, format!("Directory \"a\" already exists"));
    ensure!(res.code == SUCCESS);
    Ok(())
}

#[test]
fn noops_on_file_that_already_exists() -> Result<()> {
    let env = env(&["a"])?;
    env.write("a", "")?;
    let res = new().args(&["a"]).env(&env).run()?;
    eq!(res.output, format!("File \"a\" already exists"));
    ensure!(res.code == SUCCESS);
    Ok(())
}
