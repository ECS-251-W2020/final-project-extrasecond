mod aarch64;

pub use aarch64::sync::*;
pub use aarch64::*;

/// Architectural privilege level.
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Unknown,
}
