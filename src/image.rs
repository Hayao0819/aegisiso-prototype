use crate::error::ArchisoError;
use std::path::Path;
use crate::utils::run_command;

/// Create SquashFS image from root directory
pub async fn squash(root: &Path, out: &Path) -> Result<(), ArchisoError> {
    run_command("mksquashfs", &[root.to_str().unwrap(), out.to_str().unwrap(), "-noappend", "-comp", "xz"]).await?;
    Ok(())
}

/// Create ISO from work directory
pub async fn make_iso(src: &Path, iso: &Path, volid: &str) -> Result<(), ArchisoError> {
    run_command("xorriso", &[
        "-as", "mkisofs", "-o",
        iso.to_str().unwrap(),
        "-J", "-R", "-V", volid,
        src.to_str().unwrap()
    ]).await?;
    Ok(())
}
