/// Format of the float, storing the number of bit for each fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Format {
    /// Whether the float is signed or not, if true a bit will be assigned for the sign.
    pub signed: bool,

    /// Number of bits for the exponent.
    pub exp: u32,

    /// Number of bits for the mantissa (significand).
    pub mant: u32,

    /// The excess (offset, biased) value for the exponent.
    /// This is the value that is subtracted from the exponent to get the actual exponent.
    pub exp_excess: i32,
}

impl Format {
    /// Create from the given values for `exp`, `mant`, and `exp_excess`, default to signed.
    pub fn new(exp: u32, mant: u32, exp_excess: i32) -> Format {
        Format {
            signed: true,
            exp,
            mant,
            exp_excess,
        }
    }

    /// Create from the given values for `exp`, `mant`, and `exp_excess`, default to unsigned.
    pub fn new_unsigned(exp: u32, mant: u32, exp_excess: i32) -> Format {
        Format {
            signed: false,
            exp,
            mant,
            exp_excess,
        }
    }

    /// Create from the given values for `signed`, `exp`, `mant`, and `exp_excess`.
    pub fn new_with_sign(signed: bool, exp: u32, mant: u32, exp_excess: i32) -> Format {
        Format {
            signed,
            exp,
            mant,
            exp_excess,
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
}
