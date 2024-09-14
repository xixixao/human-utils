use anyhow::{ensure, Ok, Result};
use colored::Colorize;
use human_utils::FAILURE;

mod utils;

use crate::utils::{env, new};

#[test]
fn new_file_option_fails_with_directory_argument() -> Result<()> {
    let res = new().args(&["foo/", "--file"]).run()?;
    // TODO: Decide which output is better, but given _eq compares it's
    // probably better. Also create a watcher test runner.
    // ensure!(
    //     res.error == "Error: Fie path \"foo/\" cannot end with a / when --file option is used."
    // );
    assert_eq!(
        res.error,
        "Error: Fie path \"foo/\" cannot end with a / when --file option is used."
    );
    ensure!(res.code == FAILURE);
    Ok(())
}
