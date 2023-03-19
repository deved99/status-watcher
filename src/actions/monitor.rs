use std::process::Command;

use crate::{command_lines, Result};

const BSPC: &str = "bspc";
const WATCH_MONITOR_ARGS: [&str; 2] = ["subscribe", "monitor"];
const GET_MONITOR_ARGS: [&str; 3] = ["query", "--monitors", "--names"];

pub fn watch() -> Result<()> {
    // Print initial
    print_monitors()?;
    let iter = command_lines(BSPC, &WATCH_MONITOR_ARGS)?;
    for line_maybe in iter {
        let line = line_maybe?;
        if line.starts_with("monitor_geometry") {
            print_monitors()?;
        }
    }
    Ok(())
}

fn print_monitors() -> Result<()> {
    let cmd = Command::new(BSPC).args(GET_MONITOR_ARGS).output()?;
    let stdout = String::from_utf8(cmd.stdout)?;
    let monitors: Vec<&str> = stdout.lines().collect();
    let json = serde_json::to_string(&monitors)?;
    println!("{}", &json);
    Ok(())
}
