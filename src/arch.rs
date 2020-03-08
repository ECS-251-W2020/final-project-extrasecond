mod aarch64;

pub use aarch64::*;
pub use aarch64::sync::*;

/// Architectural privilege level.
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Unknown,
}
