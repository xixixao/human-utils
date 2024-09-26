use anyhow::{ensure, Ok, Result};
use human_utils::FAILURE;

mod utils;

use crate::utils::{env, new};

#[test]
fn cannot_create_file_and_directory_at_same_path() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["b/", "a", "b"]).env(&env).run()?;
    eq!(
        res.error,
        "Error: Cannot create both file and a directory at:\nb"
    );
    ensure!(res.code == FAILURE);
    ensure!(!env.exists("a"));
    Ok(())
}

#[test]
fn cannot_create_file_and_directory_at_same_paths() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["b/", "a", "b", "c", "c/"]).env(&env).run()?;
    eq!(
        res.error,
        "Error: Cannot create both file and a directory at:\nb\nc"
    );
    ensure!(res.code == FAILURE);
    ensure!(!env.exists("a"));
    Ok(())
}

#[test]
fn cannot_create_file_and_directory_at_same_parent_path() -> Result<()> {
    let env = env(&[])?;
    let res = new().args(&["b/c", "a", "b"]).env(&env).run()?;
    eq!(
        res.error,
        "Error: Cannot create both file and a directory at:\nb"
    );
    ensure!(res.code == FAILURE);
    ensure!(!env.exists("a"));
    Ok(())
}
