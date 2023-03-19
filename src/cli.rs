use crate::{actions, Result, Error};
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Script to simulate a 2D grid on each monitor.
pub struct Cli {
    #[argh(positional)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Commands {
    Volume,
    Monitor
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Volume => actions::volume::watch(),
            Self::Monitor => actions::monitor::watch(),
        }
    }
}

impl std::str::FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "volume" => Ok(Self::Volume),
            "monitor" => Ok(Self::Monitor),
            _ => Err(Error::InvalidCommand(s.to_string()))
        }
    }
}
