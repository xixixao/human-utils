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
            format!("{} {}", "D".bright_red(), "a/".bright_red(),),
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
            format!("{} {}", "D".bright_red(), "a/b/".bright_red(),),
            format!("{} {}", "N".bright_green(), "a/b".bright_green(),)
        )
    );
    ensure!(res.code == SUCCESS);
    eq!(env.read("a/b")?, "");
    Ok(())
}
