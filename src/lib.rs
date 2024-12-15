//! # Plantower PMSA003I Driver for Embedded HAL
//! Driver for the PMSA003I particulate sensor using
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal).
//!
//! Works on `no_std` and supports both async and blocking operation.
//!
//! See [Pmsa003i] for an example of how to use the library.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
#![deny(unsafe_code)]
#![no_std]

mod error;
mod pmsa003i;
mod types;

pub use crate::error::*;
pub use crate::pmsa003i::*;
pub use crate::types::*;

#[cfg(feature = "aqi")]
pub use aqi::{AirQuality, AirQualityError, AirQualityLevel};
