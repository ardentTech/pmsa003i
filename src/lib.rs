//! Driver for the PMSA003I particulate sensor using
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal).
//!
//! Works on `no_std` and supports both async and blocking operation.
//! To enable async, use feature `async`.
//!
//! See pmsa003i.rs rustdoc for an example of how to use the library.

#![deny(unsafe_code)]
#![no_std]
#![doc = include_str!("../README.md")]

mod error;
mod pmsa003i;
mod types;

pub use crate::error::*;
pub use crate::pmsa003i::*;
pub use crate::types::*;
