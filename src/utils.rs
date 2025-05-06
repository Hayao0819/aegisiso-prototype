use crate::error::ArchisoError;
use async_process::{Command, Stdio};

pub async fn run_command(command: &str, args: &[&str]) -> Result<(), ArchisoError> {
    let status = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(ArchisoError::Process(format!(
            "{} failed: {}",
            command, status
        )));
    }

    Ok(())
}
