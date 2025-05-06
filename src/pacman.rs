use crate::config::PacmanSection;
use crate::error::ArchisoError;
use std::path::Path;
use crate::utils::run_command;

pub async fn install_official(pac: &PacmanSection, work_dir: &Path) -> Result<(), ArchisoError> {
    let rootfs = work_dir.join("airootfs");
    std::fs::create_dir_all(&rootfs)?;

    // Execute pacstrap using run_command
    let args: Vec<&str> = vec![
        "-C",
        "/etc/pacman.conf",
        "-c",
        rootfs.to_str().unwrap(),
    ];
    let mut packages: Vec<&str> = pac.packages.iter().map(|s| s.as_str()).collect();
    let mut all_args = args;
    all_args.append(&mut packages);

    run_command("pacstrap", &all_args).await?;

    Ok(())
}
