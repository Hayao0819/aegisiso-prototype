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
            fs::prepare(&cfg.paths)?;
            info!("Copying airootfs");
            fs::copy_airootfs(&cfg.paths)?;
            fs::copy_grub_cfg(&cfg.paths)?;

            info!("Installing official packages via pacstrap");
            pacman::install_official(&cfg.pacman, &cfg.paths, Path::new(&cfg.paths.work_dir))
                .await?;

            // Create SquashFS image
            info!("Creating SquashFS image");
            let rootfs = Path::new(&cfg.paths.work_dir).join("airootfs");
            let sfs = Path::new(&cfg.paths.work_dir).join("airootfs.sfs");
            image::squash(&rootfs, &sfs).await?;

            info!("Creating ISO");
            let iso_path = Path::new(&cfg.paths.out_dir)
                .join(format!("{}-{}.iso", &cfg.iso.name, &cfg.iso.version));
            image::make_iso(Path::new(&cfg.paths.work_dir), &iso_path, &cfg.iso.name).await?;

            info!("Generating checksum and detached GPG signature");
            sign::sha512_sum_to_file(&iso_path)?;
            let keyfile = cfg
                .sign
                .gpg_key
                .as_deref()
                .ok_or_else(|| ArchisoError::Process("gpg_key が設定されていません".into()))?;
            sign::sign_detached(&iso_path, keyfile)?;

            info!("ISO generated: {}", iso_path.display());
        }
    }

    Ok(())
}
