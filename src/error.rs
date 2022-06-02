use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    #[error("insufficient bits to represent the exponent")]
    InsufficientExponentBits,

    #[error("insufficient bits to represent the mantissa")]
    InsufficientMantissaBits,
}