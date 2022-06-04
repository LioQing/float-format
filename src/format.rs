use super::*;

/// Format of the float, storing the number of bit for each fields.
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct Format {
    /// Whether the float is signed or not, if true a bit will be assigned for the sign.
    pub signed: bool,

    /// Number of bits for the exponent.
    /// Currently support up to 31 bits only.
    pub exp: u8,

    /// Number of bits for the mantissa (significand).
    pub mant: usize,

    /// The excess (offset, biased) value for the exponent.
    /// This is the value that is subtracted from the exponent to get the actual exponent.
    pub excess: u32,

    #[derivative(Debug = "ignore")]
    pub interpret: Interpret,
}

impl Format {
    /// Create from the given values for `exp`, `mant`, and `excess`, default to signed.
    pub fn new(exp: u8, mant: usize, excess: u32) -> Format {
        if exp > 31 {
            panic!("exponent bits must be less than 32");
        }

        Format {
            signed: true,
            exp,
            mant,
            excess,
            interpret: |_| None,
        }
    }

    /// Create from the given values for `exp`, `mant`, and `excess`, default to unsigned.
    pub fn new_unsigned(exp: u8, mant: usize, excess: u32) -> Format {
        if exp > 31 {
            panic!("exponent bits must be less than 32");
        }

        Format {
            signed: false,
            exp,
            mant,
            excess,
            interpret: |_| None,
        }
    }

    /// Create from the given values for `signed`, `exp`, `mant`, and `excess`.
    pub fn new_with_sign(signed: bool, exp: u8, mant: usize, excess: u32) -> Format {
        if exp > 31 {
            panic!("exponent bits must be less than 32");
        }
        
        Format {
            signed,
            exp,
            mant,
            excess,
            interpret: |_| None,
        }
    }

    /// Create from the given values for `exp` and `mant`, default to signed.
    /// The excess value is set to `(1 << (exp - 1)) - 1` (1 less than 2 to the power of `exp` - 1).
    pub fn new_ieee_excess(exp: u8, mant: usize) -> Format {
        if exp > 31 {
            panic!("exponent bits must be less than 32");
        }
        
        Format {
            signed: true,
            exp,
            mant,
            excess: (1 << (exp - 1)) - 1,
            interpret: |_| None,
        }
    }

    /// Create from the given values for `signed`, `exp`, and `mant`.
    /// The excess value is set to `(1 << (exp - 1)) - 1` (1 less than 2 to the power of `exp` - 1).
    pub fn new_ieee_excess_with_sign(signed: bool, exp: u8, mant: usize) -> Format {
        if exp > 31 {
            panic!("exponent bits must be less than 32");
        }
        
        Format {
            signed,
            exp,
            mant,
            excess: (1 << (exp - 1)) - 1,
            interpret: |_| None,
        }
    }

    /// Get the number of bits for the format.
    pub fn len(&self) -> usize {
        self.signed as usize + self.exp as usize + self.mant
    }
}

impl IeeeBinary for Format {
    /// The exponent is 8 bits and biased by 127, and the mantissa is 23 bits.
    fn ieee_binary32() -> Self {
        Self {
            interpret: Interpret::ieee_binary32(),
            ..Self::new(8, 23, 127)
        }
    }
    
    /// The exponent is 11 bits and biased by 1023, and the mantissa is 52 bits.
    fn ieee_binary64() -> Self {
        Self {
            interpret: Interpret::ieee_binary64(),
            ..Self::new(11, 52, 1023)
        }
    }
}
