use crate::*;

/// Components of a floating point number.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Components {
    /// The sign of the number, `Some(true)` if negative, `None` if the format is unsigned.
    pub sign: Option<bool>,

    /// The exponent bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub exp: BitPattern,

    /// The mantissa bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub mant: BitPattern,
}

impl Components {
    /// Create from the given values for `sign`, `exp`, and `mant`.
    /// The radix of `exp` and `mant` is deduced from the first 2 chars.
    /// '0b' => binary, '0x' => hexadecimal, '0o' => octal, '0d' => decimal.
    pub fn new(sign: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            sign,
            exp: BitPattern::from_str(exp)?,
            mant: BitPattern::from_str(mant)?,
        })
    }

    /// Create from the given values for `sign`, `exp`, and `mant`.
    /// The `exp` and `mant` should be string with '1's and '0's.
    /// Any characters other than '0' and '1' are ignored.
    pub fn new_bin(sign: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            sign,
            exp: BitPattern::from_bin_str(exp),
            mant: BitPattern::from_bin_str(mant),
        })
    }

    /// Create from the given values for `sign`, `exp`, and `mant`.
    /// The `exp` and `mant` should be string with octal digits.
    /// Any characters other than octal digits are ignored.
    pub fn new_oct(sign: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            sign,
            exp: BitPattern::from_oct_str(exp),
            mant: BitPattern::from_oct_str(mant),
        })
    }

    /// Create from the given values for `sign`, `exp`, and `mant`.
    /// The `exp` and `mant` should be string with decimal digits.
    /// Any characters other than decimal digits are ignored.
    pub fn new_dec(sign: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            sign,
            exp: BitPattern::from_dec_str(exp),
            mant: BitPattern::from_dec_str(mant),
        })
    }

    /// Create from the given values for `sign`, `exp`, and `mant`.
    /// The `exp` and `mant` should be string with hexadecimal digits.
    /// Any characters other than hexadecimal digits are ignored.
    pub fn new_hex(sign: Option<bool>, exp: &str, mant: &str) -> Result<Self, error::Error> {
        Ok(Components {
            sign,
            exp: BitPattern::from_hex_str(exp),
            mant: BitPattern::from_hex_str(mant),
        })
    }

    /// Get the format of the components with the given `excess`.
    pub fn format_with_excess(&self, excess: u32) -> Format {
        Format::new_with_sign(
            self.sign.is_some(),
            self.exp.len() as u8,
            self.mant.len(),
            excess,
        )
    }

    /// Get the format of the components.
    /// The excess value is default to 0.
    pub fn format(&self) -> Format {
        self.format_with_excess(0)
    }

    /// Get the number of bits for the format.
    pub fn len(&self) -> usize {
        self.sign.is_some() as usize + self.exp.len() + self.mant.len()
    }

    /// Get the sign bit as a str of ether '1' or '0'.
    pub fn get_sign(&self) -> Option<&str> {
        self.sign.as_ref().map(|s| if *s { "1" } else { "0" })
    }
}

impl std::fmt::Debug for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Components")
            .field("sign", &format_args!("{}", &self.sign.map(|b| if b { "-" } else { "+" }).unwrap_or("None")))
            .field("exp", &format_args!("{}", &self.exp.to_bin_string()))
            .field("mant", &format_args!("{}", &self.mant.to_bin_string()))
            .finish()
    }
}
