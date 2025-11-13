use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

/// Extract OCI image layers to create a rootfs
pub async fn create_rootfs(layer_paths: &[PathBuf], output_dir: &Path) -> Result<()> {
    log::info!("Converting image to VM rootfs...");

    // Create the output directory
    tokio::fs::create_dir_all(output_dir)
        .await
        .context("Failed to create rootfs directory")?;

    // Extract each layer in order
    for (i, layer_path) in layer_paths.iter().enumerate() {
        log::info!("Extracting layer {}/{}", i + 1, layer_paths.len());
        extract_layer(layer_path, output_dir)?;
    }

    log::info!("Rootfs creation complete");
    Ok(())
}

/// Extract a single layer (tar.gz) to the output directory
fn extract_layer(layer_path: &Path, output_dir: &Path) -> Result<()> {
    let file = File::open(layer_path).context("Failed to open layer file")?;

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    // Set to preserve permissions
    archive.set_preserve_permissions(true);
    archive.set_preserve_mtime(true);
    archive.set_unpack_xattrs(true);

    // Unpack the archive
    archive
        .unpack(output_dir)
        .context("Failed to unpack layer")?;

    Ok(())
}

/// Prepare the rootfs for VM boot by ensuring necessary directories and files exist
pub async fn prepare_rootfs_for_boot(rootfs_dir: &Path) -> Result<()> {
    log::info!("Preparing rootfs for VM boot...");

    // Ensure necessary directories exist
    let dirs = ["dev", "proc", "sys", "tmp", "run", "var/log"];

    for dir in dirs {
        let path = rootfs_dir.join(dir);
        if !path.exists() {
            tokio::fs::create_dir_all(&path)
                .await
                .context(format!("Failed to create directory: {}", dir))?;
        }
    }

    // Create a simple init script for auto-login
    let init_script = rootfs_dir.join("init.sh");
    let init_content = r#"#!/bin/sh
# Simple init script for distrovmm
mount -t proc proc /proc
mount -t sysfs sys /sys
mount -t devtmpfs dev /dev
mount -t tmpfs tmpfs /tmp

# Auto-login as root
exec /bin/bash
"#;

    tokio::fs::write(&init_script, init_content)
        .await
        .context("Failed to write init script")?;

    // Make init script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&init_script).await?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(&init_script, perms).await?;
    }

    Ok(())
}
