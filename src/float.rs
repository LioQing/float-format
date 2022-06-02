use super::*;
use bitvec::prelude::*;

/// A floating point number, also contains the format information.
#[derive(Debug, Clone)]
pub struct Float {
    pub format: Format,
    pub bits: BitVec<usize, Msb0>,
}

impl Float {
    /// Create from the given format and bit pattern.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the float.
    /// * `bits` - The bit pattern of the float.
    pub fn from_bitvec(format: Format, bits: BitVec<usize, Msb0>) -> Float {
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
        let mut bits = BitVec::<usize, Msb0>::new();

        if let Some(neg) = comps.neg {
            bits.push(neg);
        }

        let exp_len = comps.exp.len();
        let mant_len = comps.mant.len();

        let exp = comps.exp.chars().map(|c| c == '1');
        let mant = comps.mant.chars().map(|c| c == '1');

        if exp_len < format.exp as usize {
            bits.extend(std::iter::repeat(false).take(format.exp as usize - exp_len));
            bits.extend(exp);
        } else if exp.clone().take(exp_len - format.exp as usize).any(|b| b == true) {
            return Err(error::Error::InsufficientExponentBits);
        } else {
            bits.extend(exp.skip(exp_len - format.exp as usize));
        }

        if mant_len < format.mant as usize {
            bits.extend(std::iter::repeat(false).take(format.mant as usize - mant_len));
            bits.extend(mant);
        } else if mant.clone().take(mant_len - format.mant as usize).any(|b| b == true) {
            return Err(error::Error::InsufficientMantissaBits);
        } else {
            bits.extend(mant.skip(mant_len - format.mant as usize));
        }

        Ok(Float::from_bitvec(format, bits))
    }

    /// Create from the given field bit patterns.
    /// 
    /// # Arguments
    /// 
    /// * `neg` - Whether the number is signed and the sign.
    /// * `exp` - The exponent bit pattern of the number.
    /// * `mant` - The mantissa bit pattern of the number.
    pub fn from_fields(neg: Option<bool>, exp: String, mant: String) -> Result<Float, error::Error> {
        Float::from_comps(
            Format::ieee_binary32(),
            Components {
                neg,
                exp,
                mant,
            }
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

        let exp = self.bits[exp_range]
            .into_iter()
            .map(|b| if b == true { '1' } else { '0' })
            .collect::<String>();

        let mant = self.bits[mant_range]
            .into_iter()
            .map(|b| if b == true { '1' } else { '0' })
            .collect::<String>();

        Components { neg, exp, mant }
    }
}

impl From<f32> for Float {
    /// Create from the given `f32`, using IEEE binary32 format.
    fn from(f: f32) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary32();
        Float::from_bitvec(format, bits.view_bits::<Msb0>().iter().collect())
    }
}

impl Into<f32> for Float {
    /// Convert into a `f32`.
    /// May cause a lost of information.
    /// May panic.
    fn into(self) -> f32 {
        let Components { neg, exp, mant } = self.to_comps();

        let exp = i32::from_str_radix(&exp, 2).unwrap();
        let mant = u128::from_str_radix(&mant, 2).unwrap();

        let sign = match neg {
            Some(true) => -1f32,
            Some(false) => 1f32,
            None => 1f32,
        };

        let exp = 2f32.powi((exp - self.format.exp_excess) as i32);
        let mant = mant as f32 / (2f32.powi(self.format.mant as i32)) + 1f32;

        sign * exp * mant
    }
}

impl From<f64> for Float {
    /// Create from the given `f64`, using IEEE binary64 format.
    fn from(f: f64) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary64();
        Float::from_bitvec(format, bits.view_bits::<Msb0>().iter().collect())
    }
}

impl Into<f64> for Float {
    /// Convert into a `f64`.
    /// May cause a lost of information.
    /// May panic.
    fn into(self) -> f64 {
        let Components { neg, exp, mant } = self.to_comps();

        let exp = i32::from_str_radix(&exp, 2).unwrap();
        let mant = u128::from_str_radix(&mant, 2).unwrap();

        let sign = match neg {
            Some(true) => -1f64,
            Some(false) => 1f64,
            None => 1f64,
        };

        let exp = 2f64.powi((exp - self.format.exp_excess) as i32);
        let mant = mant as f64 / (2f64.powi(self.format.mant as i32)) + 1f64;

        sign * exp * mant
    }
}
