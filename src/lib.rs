mod actions;
mod cli;
mod error;

pub use error::{Error, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn main() {
    let args: cli::Cli = argh::from_env();
    args.run().unwrap();
}

pub fn command_lines(cmd: &str, args: &[&str]) -> Result<impl Iterator<Item = Result<String>>> {
    let cmd = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = BufReader::new(cmd.stdout.unwrap());
    let iter = stdout.lines().map(|x| x.map_err(Error::from));
    Ok(iter)
}
