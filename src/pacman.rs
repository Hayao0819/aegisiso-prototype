use crate::config::{PacmanSection, PathSection};
use crate::error::ArchisoError;
use crate::utils::run_command;
use std::path::{Path, PathBuf};

pub async fn install_official(
    pac: &PacmanSection,
    paths: &PathSection,
    work_dir: &Path,
) -> Result<(), ArchisoError> {
    let rootfs = work_dir.join("airootfs");
    std::fs::create_dir_all(&rootfs)?;

    let pacman_conf_path = PathBuf::from(&paths.profile).join("pacman.conf");
    let pacman_conf_str = pacman_conf_path.to_str().unwrap();

    // Execute pacstrap using run_command
    let args: Vec<&str> = vec!["-C", pacman_conf_str, "-c", rootfs.to_str().unwrap()];
    let mut packages: Vec<&str> = pac.packages.iter().map(|s| s.as_str()).collect();
    let mut all_args = args;
    all_args.append(&mut packages);

    run_command("pacstrap", &all_args).await?;

    Ok(())
}
