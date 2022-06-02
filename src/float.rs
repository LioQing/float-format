use crate::*;

/// A floating point number, also contains the format information.
#[derive(Debug, Clone, Hash)]
pub struct Float {
    pub format: Format,
    pub bits: BitPattern,
}

impl Float {
    /// Create from the given format and bit pattern.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the float.
    /// * `bits` - The bit pattern of the float.
    pub fn from_bits(format: Format, bits: BitPattern) -> Float {
        Float {
            format,
            bits,
        }
    }

    /// Create from the given format and components.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the float.
    /// * `comps` - The components of the float.
    pub fn from_comps(format: Format, comps: Components) -> Result<Float, error::Error> {
        let mut bits = BitPattern::new();

        let comps_format = comps.format();

        if format.signed != comps_format.signed {
            return Err(error::Error::MismatchedSignBit);
        }

        if let Some(neg) = comps.neg {
            bits.push(neg);
        }

        let exp = comps.exp.iter();
        let mant = comps.mant.iter();

        if comps_format.exp < format.exp {
            bits.extend(std::iter::repeat(false).take((format.exp - comps_format.exp) as usize));
            bits.extend(exp);
        } else if exp
            .clone()
            .take((comps_format.exp - format.exp) as usize)
            .any(|b| b == true)
        {
            return Err(error::Error::InsufficientExponentBits);
        } else {
            bits.extend(exp.skip((comps_format.exp - format.exp) as usize));
        }

        if comps_format.mant < format.mant {
            bits.extend(std::iter::repeat(false).take((format.mant - comps_format.mant) as usize));
            bits.extend(mant);
        } else if mant
            .clone()
            .take((comps_format.mant - format.mant) as usize)
            .any(|b| b == true)
        {
            return Err(error::Error::InsufficientMantissaBits);
        } else {
            bits.extend(mant.skip((comps_format.mant - format.mant) as usize));
        }

        Ok(Float::from_bits(format, bits))
    }

    /// Create from the given field bit patterns.
    /// The format is automatically deduced from the components.
    /// 
    /// # Arguments
    /// 
    /// * `neg` - Whether the number is signed and the sign.
    /// * `exp` - The exponent bit pattern of the number.
    /// * `mant` - The mantissa bit pattern of the number.
    /// * `excess` - The excess value of the number.
    pub fn from_fields(neg: Option<bool>, exp: &str, mant: &str, excess: i32) -> Result<Float, error::Error> {
        let comps = Components::new(
            neg,
            exp,
            mant,
        )?;

        Float::from_comps(
            comps.format_with_excess(excess),
            comps
        )
    }
    
    /// Decompose into components.
    pub fn to_comps(&self) -> Components {
        let signed = self.format.signed;
        let exp_range = signed as usize..(signed as usize + self.format.exp as usize);
        let mant_range = exp_range.end..(exp_range.end + self.format.mant as usize);

        let neg = match signed {
            true => Some(self.bits[0]),
            false => None,
        };

        let exp = self.bits[exp_range].to_owned();
        let mant = self.bits[mant_range].to_owned();

        Components { neg, exp, mant }
    }
    
    /// Create a `f32` from the given `Float`.
    /// The result may has a lost of information.
    pub fn as_f32(&self) -> f32 {
        let Components { neg, exp, mant } = self.to_comps();

        let exp = i32::from_str_radix(&exp.into_bin_string(), 2).unwrap();
        let mant = u128::from_str_radix(&mant.into_bin_string(), 2).unwrap();

        let sign = match neg {
            Some(true) => -1f32,
            Some(false) => 1f32,
            None => 1f32,
        };

        let exp = 2f32.powi((exp - self.format.excess) as i32);
        let mant = mant as f32 / (2f32.powi(self.format.mant as i32)) + 1f32;

        sign * exp * mant
    }

    /// Create a `f64` from the given `Float`.
    /// The result may has a lost of information.
    pub fn as_f64(&self) -> f64 {
        let Components { neg, exp, mant } = self.to_comps();

        let exp = i32::from_str_radix(&exp.into_bin_string(), 2).unwrap();
        let mant = u128::from_str_radix(&mant.into_bin_string(), 2).unwrap();

        let sign = match neg {
            Some(true) => -1f64,
            Some(false) => 1f64,
            None => 1f64,
        };

        let exp = 2f64.powi((exp - self.format.excess) as i32);
        let mant = mant as f64 / (2f64.powi(self.format.mant as i32)) + 1f64;

        sign * exp * mant
    }
}

impl From<f32> for Float {
    /// Create from the given `f32`, using IEEE binary32 format.
    fn from(f: f32) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary32();
        Float::from_bits(format, BitPattern::from_value(bits))
    }
}

impl From<f64> for Float {
    /// Create from the given `f64`, using IEEE binary64 format.
    fn from(f: f64) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary64();
        Float::from_bits(format, BitPattern::from_value(bits))
    }
}
