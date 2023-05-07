use std::process::Command;

use crate::{command_lines, Error, Result};

const BSPC: &str = "bspc";
const WATCH_DESKTOP_ARGS: [&str; 3] = ["subscribe", "node", "desktop"];

pub fn watch() -> Result<()> {
    // Print initial
    print_desktops()?;
    let iter = command_lines(BSPC, &WATCH_DESKTOP_ARGS)?
        .flat_map(|s| s.ok())
        .filter(|s| s.starts_with("node_stack") || s.starts_with("desktop_focus"));
    for _ in iter {
        print_desktops()?
    }
    Ok(())
}

fn print_desktops() -> Result<()> {
    Command::new("bspwm-grid")
        .arg("get-desktop")
        .status()
        .map(|_| ())
        .map_err(Error::from)
}
