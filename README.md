# Float Format

![crates.io](https://img.shields.io/crates/v/float-format)
![downloads](https://img.shields.io/crates/d/float-format)
![docs](https://img.shields.io/docsrs/float-format)
![build](https://img.shields.io/github/workflow/status/LioQing/float-format/Rust)
![size](https://img.shields.io/github/repo-size/LioQing/float-format)
![license](https://img.shields.io/crates/l/float-format)

This is a crate for customizing the format of floating point numbers.
This crate is still work in progress, many features are still to be added.

So far everything is unstable.

```rust
use float_format::*;

// Create with a custom format and parse from a string.
let float = Float::from_str(
    Format::new_ieee_excess(16, 64),
    "123456.789012345",
).unwrap();

// Format the float with custom precision.
assert_eq!(format!("{:.10}", float), "123456.789012345");

// Convert from primitive float types.
assert_eq!(Float::from(0.2f32).to_f32(), 0.2f32);
assert_eq!(Float::from(0.2f64).to_f64(), 0.2f64);
```
