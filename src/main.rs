mod cli;
mod config;
mod error;
mod fs;
mod image;
mod pacman;
mod sign;
mod template;
mod utils;

use crate::cli::{BuildMode, Cli};
use crate::config::Config;
use crate::error::ArchisoError;
use clap::Parser;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), ArchisoError> {
    let cli = Cli::parse();
    let cfg = Config::load(&cli.config)?;

    match cli.mode {
        BuildMode::Iso => {
            // Prepare work and output directories
            fs::prepare(&cfg.paths)?;
            fs::copy_airootfs(&cfg.paths)?;

            // // Install official packages via pacstrap
            pacman::install_official(&cfg.pacman, Path::new(&cfg.paths.work_dir)).await?;

            // // Generate GRUB configuration
            let grub_cfg = template::render_grub(&cfg.iso)?;
            fs::write_grub_cfg(&cfg.paths, &grub_cfg)?;

            // Create SquashFS image
            let rootfs = Path::new(&cfg.paths.work_dir).join("airootfs");
            let sfs = Path::new(&cfg.paths.work_dir).join("airootfs.sfs");
            image::squash(&rootfs, &sfs).await?;

            // // Create ISO
            let iso_path = Path::new(&cfg.paths.out_dir)
                .join(format!("{}-{}.iso", &cfg.iso.name, &cfg.iso.version));
            image::make_iso(Path::new(&cfg.paths.work_dir), &iso_path, &cfg.iso.name).await?;

            // Generate checksum and detached GPG signature
            sign::sha512_sum_to_file(&iso_path)?;
            let keyfile = cfg
                .sign
                .gpg_key
                .as_deref()
                .ok_or_else(|| ArchisoError::Process("gpg_key が設定されていません".into()))?;
            sign::sign_detached(&iso_path, keyfile)?;

            println!("ISO generated: {}", iso_path.display());
        }
    }

    Ok(())
}
