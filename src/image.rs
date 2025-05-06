use crate::error::ArchisoError;
use crate::utils::run_command;
use std::path::Path;

/// Create SquashFS image from root directory
pub async fn squash(root: &Path, out: &Path) -> Result<(), ArchisoError> {
    run_command(
        "mksquashfs",
        &[
            root.to_str().unwrap(),
            out.to_str().unwrap(),
            "-noappend",
            "-comp",
            "xz",
        ],
    )
    .await?;
    Ok(())
}

/// Create ISO from work directory
pub async fn make_iso(src: &Path, iso: &Path, volid: &str) -> Result<(), ArchisoError> {
    run_command(
        "xorriso",
        &[
            // Emulate mkisofs
            "-as",
            "mkisofs",
            // Generate Joliet directory information
            "-joliet",
            "-joliet-long",
            // Allow full 31 character filenames for ISO9660 names
            "-full-iso9660-filenames",
            // Output file
            "-o",
            iso.to_str().unwrap(),
            // Label
            "-volid",
            volid,
            // Generate rationalized Rock Ridge directory information
            "-rational-rock",
            "-V",
             // Source
            src.to_str().unwrap(),
        ],
    )
    .await?;
    Ok(())
}
