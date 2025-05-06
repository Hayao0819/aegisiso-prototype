mod cli;
mod config;
mod error;
mod fs;
mod image;
mod pacman;
mod sign;
mod utils;

use crate::cli::{BuildMode, Cli};
use crate::config::Config;
use crate::error::ArchisoError;
use clap::Parser;
use std::path::Path;

use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<(), ArchisoError> {
    env_logger::init();

    let cli = Cli::parse();
    let cfg = Config::load(&cli.config)?;

    match cli.mode {
        BuildMode::Iso => {
            info!("Preparing work and output directories");
            if let Err(e) = fs::prepare(&cfg.paths) {
                error!("Failed to prepare directories: {}", e);
                return Err(e);
            }
            info!("Copying airootfs");
            if let Err(e) = fs::copy_airootfs(&cfg.paths) {
                error!("Failed to copy airootfs: {}", e);
                return Err(e);
            }
            if let Err(e) = fs::copy_grub_cfg(&cfg.paths) {
                error!("Failed to copy grub.cfg: {}", e);
                return Err(e);
            }

            info!("Installing official packages via pacstrap");
            if let Err(e) = pacman::install_official(&cfg.pacman, &cfg.paths, Path::new(&cfg.paths.work_dir)).await {
                error!("Failed to install official packages: {}", e);
                return Err(e);
            }

            // Create SquashFS image
            info!("Creating SquashFS image");
            let rootfs = Path::new(&cfg.paths.work_dir).join("airootfs");
            let sfs = Path::new(&cfg.paths.work_dir).join("airootfs.sfs");
            if let Err(e) = image::squash(&rootfs, &sfs).await {
                error!("Failed to create SquashFS image: {}", e);
                return Err(e);
            }

            info!("Creating ISO");
            let iso_path = Path::new(&cfg.paths.out_dir)
                .join(format!("{}-{}.iso", &cfg.iso.name, &cfg.iso.version));
            if let Err(e) = image::make_iso(Path::new(&cfg.paths.work_dir), &iso_path, &cfg.iso.name).await {
                error!("Failed to create ISO: {}", e);
                return Err(e);
            }

            info!("Generating checksum and detached GPG signature");
            if let Err(e) = sign::sha512_sum_to_file(&iso_path) {
                error!("Failed to generate checksum: {}", e);
                return Err(e);
            }
            let keyfile = cfg
                .sign
                .gpg_key
                .as_deref()
                .ok_or_else(|| ArchisoError::Process("gpg_key が設定されていません".into()))?;
            if let Err(e) = sign::sign_detached(&iso_path, keyfile) {
                error!("Failed to generate detached GPG signature: {}", e);
                return Err(e);
            }

            info!("ISO generated: {}", iso_path.display());
        }
    }

    Ok(())
}
