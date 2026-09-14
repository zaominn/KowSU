// SPDX-License-Identifier: GPL-3.0-only
//
// SUSFS capability probing adapted from ReSukiSU's GPLv3 userspace interface.
// The command intentionally remains available on non-SUSFS kernels and reports
// "unsupported", allowing one KowSU manager/daemon pair to support both kinds
// of GKI kernel.

use libc::SYS_reboot;

const KSU_INSTALL_MAGIC1: u32 = 0xDEAD_BEEF;
const SUSFS_MAGIC: u32 = 0xFAFA_FAFA;
const CMD_SUSFS_SHOW_VERSION: u32 = 0x555e1;
const CMD_SUSFS_SHOW_ENABLED_FEATURES: u32 = 0x555e2;
const ERR_CMD_NOT_SUPPORTED: i32 = 126;
const SUSFS_MAX_VERSION_BUFSIZE: usize = 16;
const SUSFS_ENABLED_FEATURES_SIZE: usize = 8192;

#[repr(C)]
struct SusfsVersion {
    version: [u8; SUSFS_MAX_VERSION_BUFSIZE],
    error: i32,
}

#[repr(C)]
struct SusfsFeatures {
    features: [u8; SUSFS_ENABLED_FEATURES_SIZE],
    error: i32,
}

fn bytes_to_string(bytes: &[u8]) -> Option<String> {
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    let value = std::str::from_utf8(&bytes[..end]).ok()?.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

pub fn version() -> Option<String> {
    let mut result = SusfsVersion {
        version: [0; SUSFS_MAX_VERSION_BUFSIZE],
        error: ERR_CMD_NOT_SUPPORTED,
    };

    unsafe {
        libc::syscall(
            SYS_reboot,
            KSU_INSTALL_MAGIC1,
            SUSFS_MAGIC,
            CMD_SUSFS_SHOW_VERSION,
            &mut result,
        );
    }

    (result.error == 0)
        .then(|| bytes_to_string(&result.version))
        .flatten()
}

pub fn features() -> Option<String> {
    let mut result = Box::new(SusfsFeatures {
        features: [0; SUSFS_ENABLED_FEATURES_SIZE],
        error: ERR_CMD_NOT_SUPPORTED,
    });

    unsafe {
        libc::syscall(
            SYS_reboot,
            KSU_INSTALL_MAGIC1,
            SUSFS_MAGIC,
            CMD_SUSFS_SHOW_ENABLED_FEATURES,
            &mut *result,
        );
    }

    (result.error == 0)
        .then(|| bytes_to_string(&result.features))
        .flatten()
}
