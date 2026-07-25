#[cfg(target_os = "none")]
pub mod microcontroller;
#[cfg(not(target_os = "none"))]
pub mod other;
