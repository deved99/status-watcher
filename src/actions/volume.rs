use std::process::Command;

use lazy_regex::regex;
use serde::Serialize;

use crate::{command_lines, Result};

const GET_VOLUME_ARGS: [&str; 2] = ["get-sink-volume", "@DEFAULT_SINK@"];
const GET_MUTE_ARGS: [&str; 2] = ["get-sink-mute", "@DEFAULT_SINK@"];
const GET_MIC_VOLUME_ARGS: [&str; 2] = ["get-source-volume", "@DEFAULT_SOURCE@"];
const GET_MIC_MUTE_ARGS: [&str; 2] = ["get-source-mute", "@DEFAULT_SOURCE@"];
#[derive(Serialize)]
struct Status {
    volume: usize,
    mute: bool,
    mic_volume: usize,
    mic_mute: bool,
}

impl Status {
    fn new() -> Result<Self> {
        // Get values for speaker
        let volume = extract_volume_from_pactl(GET_VOLUME_ARGS)?;
        let mute = extract_mute_from_pactl(GET_MUTE_ARGS)?;
        let mic_volume = extract_volume_from_pactl(GET_MIC_VOLUME_ARGS)?;
        let mic_mute = extract_mute_from_pactl(GET_MIC_MUTE_ARGS)?;
        Ok(Self { volume, mute, mic_volume, mic_mute })
    }

    fn print() -> Result<()> {
        let status = Self::new()?;
        let json = serde_json::to_string(&status)?;
        println!("{}", json);
        Ok(())
    }
}

fn extract_volume_from_pactl(args: [&str; 2]) -> Result<usize> {
    let cmd = Command::new("pactl").args(args).output()?;
    let stdout = String::from_utf8(cmd.stdout)?;
    let regex = regex!("([0-9]+)%");
    let volumes: Vec<usize> = regex
        .captures_iter(&stdout)
        .flat_map(|x| x.get(1))
        .flat_map(|x| x.as_str().parse().ok())
        .collect();
    let volume = match volumes.len() {
        0 => {
            eprintln!("Weird, no volume? {}", &stdout);
            0
        }
        l => volumes.iter().sum::<usize>() / l,
    };
    Ok(volume)
}

fn extract_mute_from_pactl(args: [&str; 2]) -> Result<bool> {
    let cmd = Command::new("pactl").args(args).output()?;
    let stdout = String::from_utf8(cmd.stdout)?;
    let mute = match stdout.trim() {
        "Mute: no" => false,
        "Mute: yes" => true,
        _ => {
            eprintln!(
                "Getting mute error: found {:?}, expected 'Mute: (yes,no)'",
                &stdout
            );
            false
        }
    };
    Ok(mute)
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
