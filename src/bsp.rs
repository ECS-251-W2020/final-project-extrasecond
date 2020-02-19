
#[cfg(feature = "bsp_rpi3")]
mod rpi;

#[cfg(feature = "bsp_rpi3")]
pub use rpi::*;
