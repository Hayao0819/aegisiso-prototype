use crate::error::ArchisoError;
use std::process::Command;

pub fn check_commands() -> Result<(), ArchisoError> {
    let commands = ["mksquashfs", "xorriso", "pacstrap"];

    for cmd in &commands {
        if let Err(_) = Command::new(cmd).output() {
            return Err(ArchisoError::Process(format!("コマンド {} が見つかりません", cmd).into()));
        }
    }

    Ok(())
}
