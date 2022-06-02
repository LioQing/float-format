use crate::*;

/// Components of a floating point number.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Components {
    /// The sign of the number, `Some(true)` if negative, `None` if the format is unsigned.
    pub neg: Option<bool>,

    /// The exponent bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub exp: BitPattern,

    /// The mantissa bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub mant: BitPattern,
}

impl Components {
    /// Create from the given values for `neg`, `exp`, and `mant`.
    /// The radix of `exp` and `mant` is deduced from the first 2 chars.
    /// '0b' => binary, '0x' => hexadecimal, ('0o' => octal, '0d' => decimal to be supported).
    pub fn new(neg: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            neg,
            exp: BitPattern::from_str(exp)?,
            mant: BitPattern::from_str(mant)?,
        })
    }

    /// Create from the given values for `neg`, `exp`, and `mant`.
    /// The `exp` and `mant` are assumed to be `String` with '1's and '0's.
    pub fn new_bin(neg: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            neg,
            exp: BitPattern::from_bin_str(exp)?,
            mant: BitPattern::from_bin_str(mant)?,
        })
    }

    /// Create from the given values for `neg`, `exp`, and `mant`.
    /// The `exp` and `mant` are assumed to be `String` with hexadecimal digits.
    /// Any char that is not a hexadecimal digit is ignored.
    pub fn new_hex(neg: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            neg,
            exp: BitPattern::from_hex_str(exp)?,
            mant: BitPattern::from_hex_str(mant)?,
        })
    }

    /// Get the format of the components with the given `excess`.
    pub fn format_with_excess(&self, excess: i32) -> Format {
        Format::new_with_sign(
            self.neg.is_some(),
            self.exp.len() as u32,
            self.mant.len() as u32,
            excess
        )
    }

    /// Get the format of the components.
    /// The excess value is default  to 0.
    pub fn format(&self) -> Format {
        self.format_with_excess(0)
    }

    /// Get the number of bits for the format.
    pub fn len(&self) -> usize {
        self.neg.is_some() as usize + self.exp.len() + self.mant.len()
    }
}