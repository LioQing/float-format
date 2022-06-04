use crate::*;
use bitvec::field::BitField;
use fraction::{prelude::*, Num, ToPrimitive};
use core::str::FromStr;

/// A floating point number, also contains the format information.
#[derive(Debug, Clone)]
pub struct Float {
    pub format: Format,
    pub bits: BitPattern,
}

impl Float {
    /// Create from the given format and string.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the number.
    /// * `str` - The number in decimal form.
    pub fn from_str(format: Format, s: &str) -> Result<Self, error::Error> {
        // sign
        match s.chars().next() {
            Some('0'..='9' | '+' | '-') => {},
            _ => return Err(error::Error::ParseStringError),
        }

        let sign = s.starts_with('-');
        let s = if sign { &s[1..] } else { s };

        if sign && !format.signed {
            return Err(error::Error::NegativeSign);
        }

        let frac = BigFraction::from_str(s).map_err(|_| error::Error::ParseStringError)?;

        // extract integral and fractional parts
        let (mut int, mut frac) = (
            BigUint::from(frac.clone().numer().unwrap() / frac.clone().denom().unwrap()),
            frac.fract(),
        );

        let mut int_bits = String::new();
        let mut frac_bits = String::new();

        let exp = match int > BigUint::from(0u32) {
            false if frac == BigFraction::from(0u32) => {
                -(format.excess as i128)
            },
            true => {
                let mut exp = 0i128;

                // get integral part
                while int > BigUint::from(1u32) {
                    int_bits.push(if int.clone() % BigUint::from(2u32) == BigUint::from(1u32) { '1' } else { '0' });
                    int /= 2u32;
                    exp += 1;
                }

                // get fractional part
                while frac != BigFraction::from(0u32) && int_bits.len() + frac_bits.len() < format.mant {
                    frac *= BigFraction::from(2u32);
                    frac_bits.insert(0, if frac > BigFraction::from(1u32) { '1' } else { '0' });
                    frac %= BigFraction::from(1u32);
                }

                exp
            },
            false => {
                let mut exp = 0i128;

                // remove leading zeros of fraction
                loop {
                    frac *= BigFraction::from(2u32);
                    exp -= 1;

                    if frac > BigFraction::from(1u32) {
                        frac %= BigFraction::from(1u32);
                        break;
                    }
                }

                // get fractional part
                while frac != BigFraction::from(0u32) && frac_bits.len() < format.mant {
                    frac *= BigFraction::from(2u32);
                    frac_bits.insert(0, if frac > BigFraction::from(1u32) { '1' } else { '0' });

                    frac %= BigFraction::from(1u32);
                }

                exp
            },
        };

        let len = int_bits.len() + frac_bits.len(); 

        let int_bits = int_bits
            .chars()
            .rev()
            .take(format.mant)
            .collect::<String>();
        
        let frac_bits = frac_bits
            .chars()
            .rev()
            .chain(std::iter::repeat('0').take(if len < format.mant {
                format.mant - len
            } else {
                0
            }))
            .collect::<String>();

        if exp < -(format.excess as i128) || exp >= ((1 << format.exp) - format.excess) as i128 {
            return Err(error::Error::OutOfRange);
        }

        let exp = exp + format.excess as i128;

        let signed = format.signed;
        let width = format.exp;
        Self::from_fields(
            format,
            if signed { Some(sign) } else { None },
            format!("0b{:0width$b}", exp, width = width as usize).as_str(),
            ("0b".to_owned() + &int_bits + &frac_bits).as_str(),
        )
    }

    /// Create from the given format and bit pattern.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the float.
    /// * `bits` - The bit pattern of the float.
    pub fn from_bits(format: Format, bits: BitPattern) -> Result<Float, error::Error> {
        let mut formatted_bits = BitPattern::new();

        if bits.len() < format.len() {
            formatted_bits.extend(std::iter::repeat(false).take(format.len() - bits.len()));
            formatted_bits.extend(bits.into_iter());
        } else if bits
            .iter()
            .take(bits.len() - format.len())
            .any(|b| b == true)
        {
            return Err(error::Error::InsufficientBitsForBitPattern);
        } else {
            let len = bits.len();
            formatted_bits.extend(bits.into_iter().skip(len - format.len()));
        }

        Ok(Float {
            format,
            bits: formatted_bits,
        })
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

        // sign
        if format.signed != comps_format.signed {
            return Err(error::Error::MismatchedSignBit);
        }

        if let Some(sign) = comps.sign {
            bits.push(sign);
        }

        // exp and mant
        let exp = comps.exp.into_iter();
        let mant = comps.mant.into_iter();

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
            bits.extend(std::iter::repeat(false).take(format.mant - comps_format.mant));
            bits.extend(mant);
        } else if mant
            .clone()
            .take(comps_format.mant - format.mant)
            .any(|b| b == true)
        {
            return Err(error::Error::InsufficientMantissaBits);
        } else {
            bits.extend(mant.skip(comps_format.mant - format.mant));
        }

        Ok(Float::from_bits(format, bits).unwrap())
    }

    /// Create from the given field bit patterns.
    /// The radix of `exp` and `mant` is deduced from the first 2 chars.
    /// '0b' => binary, '0x' => hexadecimal, '0o' => octal.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format of the float.
    /// * `sign` - Whether the number is signed and the sign.
    /// * `exp` - The exponent bit pattern of the number.
    /// * `mant` - The mantissa bit pattern of the number.
    /// * `excess` - The excess value of the number.
    pub fn from_fields(format: Format, sign: Option<bool>, exp: &str, mant: &str) -> Result<Float, error::Error> {
        let comps = Components::new(
            sign,
            exp,
            mant,
        )?;

        Float::from_comps(
            format,
            comps
        )
    }
    
    /// Decompose into components.
    pub fn to_comps(&self) -> Components {
        let signed = self.format.signed;
        let exp_range = signed as usize..(signed as usize + self.format.exp as usize);
        let mant_range = exp_range.end..(exp_range.end + self.format.mant as usize);

        let sign = match signed {
            true => Some(self.bits[0]),
            false => None,
        };

        let exp = self.bits[exp_range].to_owned();
        let mant = self.bits[mant_range].to_owned();

        Components { sign, exp, mant }
    }
    
    /// Create a `f32` from the given `Float`.
    /// The result may has a lost of information.
    pub fn to_f32(&self) -> f32 {
        let Components { sign, exp, mant } = self.to_comps();

        let exp = i64::from_str_radix(&exp.into_bin_string(), 2).unwrap();
        let mant = BigUint::from_str_radix(&mant.into_bin_string(), 2).unwrap();

        let sign = match sign {
            Some(true) => -1f32,
            Some(false) => 1f32,
            None => 1f32,
        };

        let exp = 2f32.powi((exp - self.format.excess as i64) as i32);
        let mant = mant.to_f32().unwrap() / num_traits::pow(2f32, self.format.mant) + 1f32;

        sign * exp * mant
    }

    /// Create a `f64` from the given `Float`.
    /// The result may has a lost of information.
    pub fn to_f64(&self) -> f64 {
        let Components { sign, exp, mant } = self.to_comps();

        let exp = i64::from_str_radix(&exp.into_bin_string(), 2).unwrap();
        let mant = BigUint::from_str_radix(&mant.into_bin_string(), 2).unwrap();

        let sign = match sign {
            Some(true) => -1f64,
            Some(false) => 1f64,
            None => 1f64,
        };

        let exp = 2f64.powi((exp - self.format.excess as i64) as i32);
        let mant = mant.to_f64().unwrap() / num_traits::pow(2f64, self.format.mant) + 1f64;

        sign * exp * mant
    }
    
    /// Create a `f32` from the given `Float`.
    /// Raw transmutation from the bit pattern.
    pub fn to_f32_raw(&self) -> f32 {
        f32::from_bits(self.bits.load_le::<u32>())
    }

    /// Create a `f64` from the given `Float`.
    /// Raw transmutation from the bit pattern.
    pub fn to_f64_raw(&self) -> f64 {
        f64::from_bits(self.bits.load_le::<u64>())
    }
}

impl From<f32> for Float {
    /// Create from the given `f32`, using IEEE binary32 format.
    fn from(f: f32) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary32();
        Float::from_bits(format, BitPattern::from_value(bits)).unwrap()
    }
}

#[cfg(target_pointer_width = "64")]
impl From<f64> for Float {
    /// Create from the given `f64`, using IEEE binary64 format.
    fn from(f: f64) -> Float {
        let bits = f.to_bits();
        let format = Format::ieee_binary64();
        Float::from_bits(format, BitPattern::from_value(bits)).unwrap()
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let comps = self.to_comps();

        if let Some(s) = (self.format.interpret)(&comps) {
            return write!(f, "{}", s);
        }

        // sign
        let sign = match comps.sign {
            Some(true) => "-",
            _ => "",
        };

        // exp
        let exp =
            BigInt::from_str_radix(&comps.exp.into_bin_string(), 2).unwrap() - self.format.excess.clone();

        let exp = match exp < BigInt::from(0i32) {
            true => BigFraction::new(BigUint::from(1u32), BigUint::from(2u32) << ((-exp).to_usize().unwrap() - 1)),
            false => BigFraction::from(BigUint::from(1u32) << exp.to_usize().unwrap()),
        };

        // mant
        let frac = comps.mant
            .iter()
            .fold((BigUint::from(0u32), BigUint::from(0u32)),
                |(numer, denom), b| {
                    (
                        numer * 2u32 + if b == true { 1u32 } else { 0u32 },
                        denom * 2u32 + 1u32,
                    )
                }
            );
        
        let frac =
            BigFraction::from(1u32)
            + BigFraction::new(frac.0, frac.1);
        
        let value = frac * exp;

        // output, by default 6 significant digits
        if let Some(prec) = f.precision() {
            return write!(f, "{}{}", sign, format!("{:.1$}", value, prec))
        }

        if value > BigFraction::from(9999999u32) {
            return write!(f, "{}{}", sign, BigUint::from(value.clone().numer().unwrap() / value.clone().denom().unwrap()))
        }

        let prec = if value > BigFraction::from(999999u32) {
            1
        } else if value > BigFraction::from(99999u32) {
            2
        } else if value > BigFraction::from(9999u32) {
            3
        } else if value > BigFraction::from(999u32) {
            4
        } else if value > BigFraction::from(99u32) {
            5
        } else if value > BigFraction::from(9u32) {
            6
        } else if value >= BigFraction::from(1u32) {
            7
        } else {
            let mut value = value.fract();
            let mut prec = 8;

            loop {
                value *= BigFraction::from(10u32);
                if value.trunc() > BigFraction::from(0u32) {
                    break prec;
                }
                prec += 1;
            }
        };

        write!(f, "{}{}", sign, format!("{:.1$}", value, prec))
    }
}
