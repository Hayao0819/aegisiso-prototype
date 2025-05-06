use crate::config::PathSection;
use crate::error::ArchisoError;
use std::path::Path;

pub fn prepare(paths: &PathSection) -> Result<(), ArchisoError> {
    std::fs::create_dir_all(&paths.work_dir)?;
    std::fs::create_dir_all(&paths.out_dir)?;
    Ok(())
}

pub fn copy_airootfs(paths: &PathSection) -> Result<(), ArchisoError> {
    let src = Path::new(&paths.profile).join("airootfs");
    let dst = Path::new(&paths.work_dir).join("airootfs");

    // remove any stale airootfs
    if dst.exists() {
        std::fs::remove_dir_all(&dst)?;
    }

    println!(
        "Copying airootfs from {} to {}",
        src.display(),
        dst.display()
    );

    // Ensure the destination base directory exists
    std::fs::create_dir_all(&dst)?;

    for entry in walkdir::WalkDir::new(&src).min_depth(1) {
        let entry = entry.map_err(|e| {
            ArchisoError::Process(format!(
                "Error walking source directory {}: {}",
                src.display(),
                e
            ))
        })?;
        let src_path = entry.path();
        // Calculate path relative to the source directory base
        let relative_path = src_path.strip_prefix(&src).map_err(|e| {
            ArchisoError::Process(format!(
                "Failed to get relative path for {}: {}",
                src_path.display(),
                e
            ))
        })?;
        // Construct the corresponding destination path
        let dst_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            // Create the directory in the destination
            std::fs::create_dir_all(&dst_path).map_err(|e| {
                ArchisoError::Process(format!(
                    "Failed to create destination directory {}: {}",
                    dst_path.display(),
                    e
                ))
            })?;
        } else if entry.file_type().is_file() {
            // Ensure parent directory exists before copying file
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ArchisoError::Process(format!(
                        "Failed to create parent directory {}: {}",
                        parent.display(),
                        e
                    ))
                })?;
            }
            // Copy the file
            std::fs::copy(&src_path, &dst_path).map_err(|e| {
                ArchisoError::Process(format!(
                    "Failed to copy file {} to {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                ))
            })?;
        }
        // Note: This doesn't explicitly handle symlinks or other special file types.
    }

    println!(
        "Copied airootfs from {} to {}",
        src.display(),
        dst.display()
    );

    Ok(())
}

pub fn copy_grub_cfg(paths: &PathSection) -> Result<(), ArchisoError> {
    let src = Path::new(&paths.profile).join("grub");
    let dst = Path::new(&paths.work_dir).join("boot/grub");

    // remove any stale airootfs
    if dst.exists() {
        std::fs::remove_dir_all(&dst)?;
    }

    println!(
        "Copying airootfs from {} to {}",
        src.display(),
        dst.display()
    );

    // Ensure the destination base directory exists
    std::fs::create_dir_all(&dst)?;

    for entry in walkdir::WalkDir::new(&src).min_depth(1) {
        let entry = entry.map_err(|e| {
            ArchisoError::Process(format!(
                "Error walking source directory {}: {}",
                src.display(),
                e
            ))
        })?;
        let src_path = entry.path();
        // Calculate path relative to the source directory base
        let relative_path = src_path.strip_prefix(&src).map_err(|e| {
            ArchisoError::Process(format!(
                "Failed to get relative path for {}: {}",
                src_path.display(),
                e
            ))
        })?;
        // Construct the corresponding destination path
        let dst_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            // Create the directory in the destination
            std::fs::create_dir_all(&dst_path).map_err(|e| {
                ArchisoError::Process(format!(
                    "Failed to create destination directory {}: {}",
                    dst_path.display(),
                    e
                ))
            })?;
        } else if entry.file_type().is_file() {
            // Ensure parent directory exists before copying file
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ArchisoError::Process(format!(
                        "Failed to create parent directory {}: {}",
                        parent.display(),
                        e
                    ))
                })?;
            }
            // Copy the file
            std::fs::copy(&src_path, &dst_path).map_err(|e| {
                ArchisoError::Process(format!(
                    "Failed to copy file {} to {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                ))
            })?;
        }
        // Note: This doesn't explicitly handle symlinks or other special file types.
    }

    println!(
        "Copied airootfs from {} to {}",
        src.display(),
        dst.display()
    );

    Ok(())
}
