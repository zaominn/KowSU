// SPDX-License-Identifier: GPL-3.0-only

// Compatibility namespace for the ReSukiSU SUSFS userspace implementation.
// KowSU keeps its Android modules at the crate root, so only the SUSFS module
// and the utility function it uses are exposed through this namespace.
pub mod susfs;

pub mod utils {
    pub use crate::utils::daemonize;
}
