// Set up cargo clippy
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::missing_errors_doc)]

extern crate core;

pub mod config;

pub mod platforms;

#[cfg(test)]
pub mod tests;
