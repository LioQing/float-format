use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    #[error("insufficient bits to represent the exponent")]
    InsufficientExponentBits,

    #[error("insufficient bits to represent the mantissa")]
    InsufficientMantissaBits,

    #[error("mismatched format and value of the sign bit")]
    MismatchedSignBit,

    #[error("invalid or missing radix prefix")]
    InvalidRadixPrefix,

    #[error("insufficient bits to store the given bit pattern")]
    InsufficientBitsForBitPattern,

    #[error("invalid number string given for parsing")]
    ParseStringError,

    #[error("number string out of range for format")]
    OutOfRange,

    #[error("negative sign for unsigned format")]
    NegativeSign,
}