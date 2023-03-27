use std::process::Command;

use crate::{command_lines, Error, Result};

const CMD: &str = "dbus-monitor";
const ARGS: [&str; 3] = ["--session", "--profile", "interface=org.freedesktop.Notifications"];

pub fn watch() -> Result<()> {
    // Print initial
    list_notifications()?;
    let iter = command_lines(CMD, &ARGS)?;
    for _ in iter {
        list_notifications()?
    }
    Ok(())
}

fn list_notifications() -> Result<()> {
    Command::new("notif")
        .arg("read")
        .status()
        .map(|_| ())
        .map_err(Error::from)
}
