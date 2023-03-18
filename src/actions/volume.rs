use std::process::Command;

use lazy_regex::regex;
use serde::Serialize;

use crate::{command_lines, Result};

const GET_VOLUME_ARGS: [&str; 2] = ["get-sink-volume", "@DEFAULT_SINK@"];
const GET_MUTE_ARGS: [&str; 2] = ["get-sink-mute", "@DEFAULT_SINK@"];

#[derive(Serialize)]
struct Status {
    volume: usize,
    mute: bool,
}

impl Status {
    fn new() -> Result<Self> {
        // Get current volum
        let cmd = Command::new("pactl").args(GET_VOLUME_ARGS).output()?;
        let stdout = String::from_utf8(cmd.stdout)?;
        let regex = regex!("([0-9]+)%");
        let volumes: Vec<usize> = regex
            .captures_iter(&stdout)
            .map(|x| x.get(1))
            .filter(|x| x.is_some())
            .map(|x| x.expect("This list should have filtered out None!"))
            .map(|x| x.as_str().parse())
            .map(|x| x.expect("The first capture group of the regex is an integer, right?"))
            .collect();
        let volume = volumes.iter().sum::<usize>() / volumes.len();
        // Get current muteness
        let cmd = Command::new("pactl").args(GET_MUTE_ARGS).output()?;
        let stdout = String::from_utf8(cmd.stdout)?;
        let mute = match stdout.trim() {
            "Mute: no" => false,
            "Mute: yes" => true,
            _ => {
                eprintln!("Getting mute error: found {:?}, expected 'Mute: (yes,no)'", &stdout);
                false
            }
        };
        Ok(Self { volume, mute })
    }

    fn print() -> Result<()> {
        let status = Self::new()?;
        let json = serde_json::to_string(&status)?;
        println!("{}", json);
        Ok(())
    }
}

pub fn watch() -> Result<()> {
    // Print initial
    Status::print()?;
    let iter = command_lines("pactl", &["subscribe"])?;
    for line in iter {
        let line = line?;
        if line.contains("'change' on source") {
            Status::print()?;
        }
    }
    Ok(())
}
