use crate::{actions, Error, Result};
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
    Desktop,
    Monitor,
    Notif,
    Volume,
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Desktop => actions::desktop::watch(),
            Self::Monitor => actions::monitor::watch(),
            Self::Notif => actions::notif::watch(),
            Self::Volume => actions::volume::watch(),
        }
    }
}

impl std::str::FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "desktop" => Ok(Self::Desktop),
            "monitor" => Ok(Self::Monitor),
            "volume" => Ok(Self::Volume),
            "notif" => Ok(Self::Notif),
            _ => Err(Error::InvalidCommand(s.to_string())),
        }
    }
}
