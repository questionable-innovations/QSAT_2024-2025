#![doc = include_str!("../README.md")]
#![no_std]
// Deny unsafe code unless the SPI debug feature is enabled
#![cfg_attr(not(feature = "debug"), deny(unsafe_code))]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]

pub mod lora;
pub mod rfm95;
