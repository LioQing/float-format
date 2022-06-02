//! # Float Format
//! 
//! This is a crate for customizing the format of floating point numbers.
//! This crate is still work in progress, many features are still to be added.

mod format;
pub use format::Format;

mod float;
pub use float::Float;

mod components;
pub use components::Components;

pub mod error;