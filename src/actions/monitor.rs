use std::process::Command;

use lazy_regex::regex;

use crate::{command_lines, Result};

const BSPC: &str = "bspc";
const WATCH_MONITOR_ARGS: [&str; 2] = ["subscribe", "monitor"];
const XRANDR: &str = "xrandr";
const GET_MONITOR_ARGS: [&str; 1] = ["--listactivemonitors"];

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
    let cmd = Command::new(XRANDR).args(GET_MONITOR_ARGS).output()?;
    let stdout = String::from_utf8(cmd.stdout)?;
    let regex = regex!(r"[0-9]+: \+\*?([^ ]+) ");
    let monitors: Vec<&str> = regex
        .captures_iter(&stdout)
        .map(|x| x.get(1))
        .filter_map(|x| x.map(|s| s.as_str()))
        .collect();
    let json = serde_json::to_string(&monitors)?;
    println!("{}", &json);
    Ok(())
}
