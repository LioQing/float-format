/// Format of the float, storing the number of bit for each fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Format {
    /// Whether the float is signed or not, if true a bit will be assigned for the sign.
    pub signed: bool,

    /// Number of bits for the exponent.
    /// Currently only support up to 64 bits.
    pub exp: usize,

    /// Number of bits for the mantissa (significand).
    /// Currently only support up to 128 bits.
    pub mant: usize,

    /// The excess (offset, biased) value for the exponent.
    /// This is the value that is subtracted from the exponent to get the actual exponent.
    pub excess: u64,
}

impl Format {
    /// Create from the given values for `exp`, `mant`, and `excess`, default to signed.
    pub fn new(exp: usize, mant: usize, excess: u64) -> Format {
        Format {
            signed: true,
            exp,
            mant,
            excess,
        }
    }

    /// Create from the given values for `exp`, `mant`, and `excess`, default to unsigned.
    pub fn new_unsigned(exp: usize, mant: usize, excess: u64) -> Format {
        Format {
            signed: false,
            exp,
            mant,
            excess,
        }
    }

    /// Create from the given values for `signed`, `exp`, `mant`, and `excess`.
    pub fn new_with_sign(signed: bool, exp: usize, mant: usize, excess: u64) -> Format {
        Format {
            signed,
            exp,
            mant,
            excess,
        }
    }

    /// Create from the given values for `exp` and `mant`, default to signed.
    /// The excess value is set to `(1 << (exp - 1)) - 1` (1 less than 2 to the power of `exp` - 1).
    pub fn new_ieee_excess(exp: usize, mant: usize) -> Format {
        Format {
            signed: true,
            exp,
            mant,
            excess: (1 << (exp - 1)) - 1,
        }
    }

    /// Create from the given values for `signed`, `exp`, and `mant`.
    /// The excess value is set to `(1 << (exp - 1)) - 1` (1 less than 2 to the power of `exp` - 1).
    pub fn new_ieee_excess_with_sign(signed: bool, exp: usize, mant: usize) -> Format {
        Format {
            signed,
            exp,
            mant,
            excess: (1 << (exp - 1)) - 1,
        }
    }

    /// Create from the IEEE binary32 format.
    /// The exponent is 8 bits and biased by 127, and the mantissa is 23 bits.
    pub fn ieee_binary32() -> Format {
        Format::new(8, 23, 127)
    }

    /// Create from the IEEE binary64 format.
    /// The exponent is 11 bits and biased by 1023, and the mantissa is 52 bits.
    pub fn ieee_binary64() -> Format {
        Format::new(11, 52, 1023)
    }

    /// Get the number of bits for the format.
    pub fn len(&self) -> usize {
        self.signed as usize + self.exp + self.mant
    }
}
