use anyhow::Result;
use rust_embed::RustEmbed;

#[cfg(target_os = "android")]
mod android {
    use crate::assets::Asset;
    use crate::defs::{BINARY_DIR, DAEMON_PATH};
    use crate::utils::ensure_binary;
    use const_format::concatcp;
    use std::os::unix::fs::MetadataExt;

    pub const RESETPROP_PATH: &str = concatcp!(BINARY_DIR, "resetprop");
    pub const KSU_SUSFS: &str = concatcp!(BINARY_DIR, "ksu_susfs");
    pub const BUSYBOX_PATH: &str = concatcp!(BINARY_DIR, "busybox");
    pub const BOOTCTL_PATH: &str = concatcp!(BINARY_DIR, "bootctl");

    fn is_managed_susfs_link() -> std::io::Result<bool> {
        let link = std::fs::symlink_metadata(KSU_SUSFS)?;
        if !link.file_type().is_file() {
            return Ok(false);
        }
        let daemon = std::fs::metadata(DAEMON_PATH)?;
        Ok(link.dev() == daemon.dev() && link.ino() == daemon.ino())
    }

    fn ensure_susfs_link() -> anyhow::Result<()> {
        match std::fs::remove_file(KSU_SUSFS) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        std::fs::hard_link(DAEMON_PATH, KSU_SUSFS)?;
        Ok(())
    }

    fn remove_susfs_link() -> anyhow::Result<()> {
        match is_managed_susfs_link() {
            Ok(true) => std::fs::remove_file(KSU_SUSFS)?,
            Ok(false) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }

    pub fn reconcile_susfs_link() -> anyhow::Result<()> {
        if crate::android::susfs::config::model::Config::read_or_default().is_enabled()
            && crate::android::susfs::api::features::show::version().is_ok()
        {
            ensure_susfs_link()
        } else {
            remove_susfs_link()
        }
    }

    pub fn ensure_binaries(ignore_if_exist: bool) -> anyhow::Result<()> {
        for file in Asset::iter() {
            if file == "ksuinit" || file.ends_with(".ko") {
                // don't extract ksuinit and kernel modules
                continue;
            }
            let asset =
                Asset::get(&file).ok_or_else(|| anyhow::anyhow!("asset not found: {file}"))?;
            ensure_binary(format!("{BINARY_DIR}{file}"), &asset.data, ignore_if_exist)?;
        }

        // Create resetprop -> ksud symlink (resetprop is now built into ksud)
        let resetprop_link = RESETPROP_PATH;
        let _ = std::fs::remove_file(resetprop_link);
        std::os::unix::fs::symlink("/data/adb/ksud", resetprop_link)?;

        // Do not replace an existing client when built-in SUSFS management is disabled.
        if crate::android::susfs::config::model::Config::read_or_default().is_enabled()
            && crate::android::susfs::api::features::show::version().is_ok()
        {
            ensure_susfs_link()?;
        }
        Ok(())
    }
}

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(all(target_arch = "arm", target_os = "android"))]
#[derive(RustEmbed)]
#[folder = "bin/arm"]
struct Asset;

#[cfg(all(target_arch = "x86_64", target_os = "android"))]
#[derive(RustEmbed)]
#[folder = "bin/x86_64"]
struct Asset;

#[cfg(all(target_arch = "aarch64", target_os = "android"))]
#[derive(RustEmbed)]
#[folder = "bin/aarch64"]
struct Asset;

// If not Android, ie. macos, linux, windows, include both
#[cfg(not(target_os = "android"))]
#[derive(RustEmbed)]
#[folder = "bin"]
struct Asset;

#[allow(unused)]
pub fn get_asset_data(name: &str) -> Result<std::borrow::Cow<'static, [u8]>> {
    let asset = Asset::get(name).ok_or_else(|| anyhow::anyhow!("asset not found: {name}"))?;
    Ok(asset.data)
}

pub fn get_asset(name: &str) -> Result<Box<dyn AsRef<[u8]>>> {
    let asset = Asset::get(name).ok_or_else(|| anyhow::anyhow!("asset not found: {name}"))?;
    Ok(Box::new(asset.data))
}

pub fn list_supported_kmi() -> std::vec::Vec<std::string::String> {
    let mut list = Vec::new();
    for file in Asset::iter() {
        // kmi_name = "xxx_kernelsu.ko"
        if let Some(kmi) = file.strip_suffix("_kernelsu.ko") {
            list.push(kmi.to_string());
        }
    }
    list
}
