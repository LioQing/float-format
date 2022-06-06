use crate::*;
use bitvec::prelude::*;
use fraction::{prelude::*, Num};

/// Trait for structure that have an IEEE binary standard form.
pub trait IeeeBinary {
    fn ieee_binary32() -> Self;
    fn ieee_binary64() -> Self;
}

/// Special interpretation of bit patterns for special values.
/// This function is called during output.
pub type Interpret = fn(&Components) -> Option<String>;

/// Function to convert components to a string according to IEEE binary32 and binary64 format.
fn ieee_interpret(comps: &Components) -> Option<String> {
    match comps {
        c if c.sign == Some(false) && c.exp.is_all_zero() && c.mant.is_all_zero() => {
            Some("0".to_owned())
        },
        c if c.sign == Some(true) && c.exp.is_all_zero() && c.mant.is_all_zero() => {
            Some("-0".to_owned())
        },
        c if c.sign == Some(false) && c.exp.is_all_one() && c.mant.is_all_zero() => {
            Some("inf".to_owned())
        },
        c if c.sign == Some(true) && c.exp.is_all_one() && c.mant.is_all_zero() => {
            Some("-inf".to_owned())
        },
        c if c.exp.is_all_one() && !c.mant.is_all_zero() && c.mant.iter().next().map(|b| *b) == Some(true) => {
            Some("NaN".to_owned())
        },
        c if c.exp.is_all_one() && !c.mant.is_all_zero() && c.mant.iter().next().map(|b| *b) == Some(false) => {
            Some("sNaN".to_owned())
        },
        _ => None,
    }
}

impl IeeeBinary for Interpret {
    fn ieee_binary32() -> Self {
        ieee_interpret
    }

    fn ieee_binary64() -> Self {
        ieee_interpret
    }
}

pub type BitPattern = BitVec<usize, Msb0>;

pub trait BitPatternExt where Self: Sized {
    fn from_value<T>(val: T) -> Self
    where
        T: BitStore;

    /// Create from the given string.
    /// The radix is deduced from the first 2 chars.
    /// '0b' => binary, '0x' => hexadecimal, '0o' => octal, '0d' => decimal.
    fn from_str(s: &str) -> Result<Self, error::Error>;

    /// Create from the given binary string.
    /// Any character other than '0' or '1' is ignored.
    fn from_bin_str(s: &str) -> Self;

    /// Create from the given decimal string.
    /// Any character other than decimal digits is ignored.
    fn from_dec_str(s: &str) -> Self;
    
    /// Create from the given octal string.
    /// Any character other than octal digits is ignored.
    fn from_oct_str(s: &str) -> Self;
    
    /// Create from the given hexadecimal string.
    /// Any character other than hexadecimal digits is ignored.
    fn from_hex_str(s: &str) -> Self;
    
    /// Check if the bit pattern is all one.
    fn is_all_one(&self) -> bool;

    /// Check if the bit pattern is all zero.
    fn is_all_zero(&self) -> bool;

    /// Convert the bit pattern to a string representing the binary value.
    fn to_bin_string(&self) -> String;

    /// Convert the bit pattern to a string representing the decimal value.
    fn to_oct_string(&self) -> String;

    /// Convert the bit pattern to a string representing the hexadecimal value.
    fn to_dec_string(&self) -> String;

    /// Convert the bit pattern to a string representing the hexadecimal value.
    fn to_hex_string(&self) -> String;
}

impl BitPatternExt for BitPattern {
    fn from_value<T>(val: T) -> Self
    where
        T: BitStore
    {
        val.view_bits::<Msb0>().iter().collect()
    }

    fn from_str(s: &str) -> Result<Self, error::Error> {
        if s.len() < 3 {
            return Err(error::Error::InvalidRadixPrefix);
        }

        match s[0..2].as_ref() {
            "0b" => Ok(Self::from_bin_str(&s[2..])),
            "0o" => Ok(Self::from_oct_str(&s[2..])),
            "0d" => Ok(Self::from_dec_str(&s[2..])),
            "0x" => Ok(Self::from_hex_str(&s[2..])),
            _ => Err(error::Error::InvalidRadixPrefix),
        }
    }
    
    fn from_bin_str(s: &str) -> Self {
        s
            .chars()
            .filter(|c| c.is_digit(2))
            .map(|c| c == '1')
            .collect()
    }

    fn from_dec_str(s: &str) -> Self {
        let s = s.chars().filter(|c| c.is_digit(10)).collect::<String>();

        let mut int = BigUint::from_str_radix(&s, 10).unwrap();
        let mut bits = String::new();

        while int > BigUint::from(0u32) {
            bits.push(if int.clone() % BigUint::from(2u32) == BigUint::from(1u32) { '1' } else { '0' });
            int /= 2u32;
        }

        let bits = bits
            .chars()
            .rev()
            .collect::<String>();

        Self::from_bin_str(&bits)
    }

    fn from_oct_str(s: &str) -> Self {
        s
            .chars()
            .filter(|c| c.is_digit(8))
            .flat_map(|c| format!("{:3b}", c.to_digit(8).unwrap())
                .chars()
                .collect::<Vec<_>>()
            )
            .map(|c| c == '1')
            .collect()
    }
    
    fn from_hex_str(s: &str) -> Self {
        s
            .chars()
            .filter(|c| c.is_digit(16))
            .flat_map(|c| format!("{:4b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
            )
            .map(|c| c == '1')
            .collect()
    }

    fn is_all_one(&self) -> bool {
        self.iter().all(|b| *b)
    }

    fn is_all_zero(&self) -> bool {
        self.iter().all(|b| !*b)
    }
    
    fn to_bin_string(&self) -> String {
        self
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect()
    }
    
    fn to_oct_string(&self) -> String {
        self
            .as_bitslice()
            .chunks(3)
            .map(|c| format!("{:o}", c)
                .chars()
                .filter(|c| c.is_digit(8))
                .collect::<String>()
            )
            .collect()
    }

    fn to_dec_string(&self) -> String {
        self
            .iter()
            .fold(BigUint::from(0u32), |acc, b| acc * 2u32 + if *b { 1u32 } else { 0u32 })
            .to_string()
    }

    fn to_hex_string(&self) -> String {
        self
            .as_bitslice()
            .chunks(4)
            .map(|c| format!("{:x}", c)
                .chars()
                .filter(|c| c.is_digit(16))
                .collect::<String>()
            )
            .collect()
    }
}
