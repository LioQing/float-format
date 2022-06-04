//! # Float Format
//! 
//! This is a crate for customizing the format of floating point numbers.
//! This crate is still work in progress, many features are still to be added.
//! 
//! So far everything is unstable.
//! 
//! ## Example
//! 
//! ```rust
//! use float_format::*;
//! 
//! fn main() {
//!     // Create with a custom format and parse from a string.
//!     let float = Float::from_str(
//!         Format::new_ieee_excess(16, 64),
//!         "123456.789012345",
//!     ).unwrap();
//! 
//!     // Format the float with custom precision.
//!     assert_eq!(format!("{:.8}", float), "123456.78901234");
//! 
//!     // Convert from primitive float types.
//!     assert_eq!(Float::from(0.2f32).to_f32(), 0.2f32);
//!     assert_eq!(Float::from(0.2f64).to_f64(), 0.2f64);
//! }
//! ```

mod format;
pub use format::Format;

mod float;
pub use float::Float;

mod components;
pub use components::Components;

mod utils;
pub use utils::*;

pub mod error;