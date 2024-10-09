use anyhow::{Context, Ok, Result};
use camino::Utf8Path;
use std::io::{BufRead, Read, Write};

// See https://github.com/rust-lang/rust/issues/46379 for all the `allow(dead_code)`s.

#[allow(dead_code)]
pub const SUCCESS: i32 = 0;

pub struct Outcome {
    #[allow(dead_code)]
    pub prompt: String,
    #[allow(dead_code)]
    pub output: String,
    #[allow(dead_code)]
    pub error: String,
    #[allow(dead_code)]
    pub code: i32,
}

pub struct Runner<'a> {
    binary: String,
    args: Vec<String>,
    env: Option<&'a Environment>,
    answer: Option<String>,
}

pub struct Environment {
    dir: tempfile::TempDir,
    description: Vec<String>,
}

pub fn env(description: &[&str]) -> Result<Environment> {
    // We stuck this in here since it's usually the first code in any test to run
    colored::control::SHOULD_COLORIZE.set_override(true);

    let env = Environment {
        dir: tempfile::tempdir().context("Could not create temp dir")?,
        description: description.iter().map(|s| s.to_string()).collect(),
    };
    create_environment(&env)?;
    return Ok(env);
}

impl Environment {
    #[allow(dead_code)]
    pub fn exists(&self, name: &str) -> bool {
        self.dir.path().join(name).symlink_metadata().is_ok()
    }

    #[allow(dead_code)]
    pub fn exists_directory(&self, name: &str) -> bool {
        self.dir
            .path()
            .join(name)
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.is_dir())
    }

    #[allow(dead_code)]
    pub fn read(&self, name: &str) -> Result<String> {
        Ok(std::fs::read_to_string(self.dir.path().join(name))?)
    }

    #[allow(dead_code)]
    pub fn write(&self, name: &str, content: &str) -> Result<()> {
        Ok(std::fs::write(self.dir.path().join(name), content)?)
    }

    #[allow(dead_code)]
    pub fn debug(&self) {
        println!(
            "{:?}",
            std::fs::read_dir(self.dir.path())
                .unwrap()
                .collect::<Vec<_>>()
        );
    }
}

#[allow(dead_code)]
pub fn mov() -> Runner<'static> {
    Runner::new("target/debug/mov")
}

#[allow(dead_code)]
pub fn del() -> Runner<'static> {
    Runner::new("target/debug/del")
}

#[allow(dead_code)]
pub fn new() -> Runner<'static> {
    Runner::new("target/debug/new")
}

impl<'a> Runner<'a> {
    pub fn new(binary: &str) -> Runner<'static> {
        Runner {
            binary: binary.to_owned(),
            args: Vec::new(),
            env: None,
            answer: None,
        }
    }

    #[allow(dead_code)]
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args = args.iter().map(|s| s.to_string()).collect();
        self
    }

    #[allow(dead_code)]
    pub fn answer(mut self, answer: &str) -> Self {
        self.answer = Some(answer.to_owned());
        self
    }

    #[allow(dead_code)]
    pub fn env(mut self, env: &'a Environment) -> Self {
        self.env = Some(env);
        self
    }

    pub fn run(self) -> Result<Outcome> {
        let default_binding = env(&[])?;
        let current_env = self.env.unwrap_or(&default_binding);
        let mut process = std::process::Command::new(
            Utf8Path::new(&self.binary)
                .canonicalize_utf8()
                .context("Failed to convert target path to canonical")?,
        )
        .args(self.args)
        .env("CLICOLOR_FORCE", "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(current_env.dir.path())
        .spawn()
        .context("Failed to execute the binary")?;
        let mut stdout = process.stdout.take().context("Failed to open stdout")?;
        let mut stderr = process.stderr.take().context("Failed to open stderr")?;
        let mut stdin = process.stdin.take().context("Failed to open stdin")?;
        let mut prompt = String::new();
        let mut output = String::new();
        let mut error = String::new();
        if let Some(answer) = self.answer {
            read_until(']', &mut prompt, &mut stdout)?;
            write!(stdin, "{}\n", answer)?;
        }
        stdout
            .read_to_string(&mut output)
            .context("Failed to read stdout")?;
        stderr
            .read_to_string(&mut error)
            .context("Failed to read stderr")?;
        let code = process
            .wait()
            .context("Binary didn't exit")?
            .code()
            .context("No code returned from binary exit")?;
        Ok(Outcome {
            prompt,
            output: output.trim().to_string(),
            error: error.trim().to_string(),
            code,
        })
    }
}

fn create_environment(env: &Environment) -> Result<()> {
    let temp_dir_path = env.dir.path();
    for file in &env.description {
        if file.contains(std::path::MAIN_SEPARATOR) {
            let path = Utf8Path::new(file)
                .parent()
                .context("Failed to get parent path")?;
            std::fs::create_dir_all(temp_dir_path.join(path))?;
        }
        std::fs::write(temp_dir_path.join(file), file).context("Failed to create file")?;
    }
    Ok(())
}

fn read_until(delimiter: char, prompt: &mut String, stdout: &mut impl Read) -> Result<()> {
    let mut buf = Vec::new();
    std::io::BufReader::new(stdout).read_until(delimiter as u8, &mut buf)?;
    prompt.push_str(std::str::from_utf8(&buf)?);
    Ok(())
}

#[macro_export]
macro_rules! eq {
    ($($tt:tt)*) => {
        pretty_assertions::assert_eq!($($tt)*)
    };
}
