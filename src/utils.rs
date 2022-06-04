use crate::*;
use bitvec::prelude::*;

pub trait IeeeBinary {
    fn ieee_binary32() -> Self;
    fn ieee_binary64() -> Self;
}

/// Special interpretation of bit patterns for special values.
/// This function is called during output.
pub type Interpret = fn(&Components) -> Option<String>;

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

    fn from_str(s: &str) -> Result<Self, error::Error>;
    fn from_bin_str(s: &str) -> Result<Self, error::Error>;
    fn from_oct_str(s: &str) -> Result<Self, error::Error>;
    fn from_hex_str(s: &str) -> Result<Self, error::Error>;
    
    fn is_all_one(&self) -> bool;
    fn is_all_zero(&self) -> bool;

    fn to_bin_string(&self) -> String;
    fn into_bin_string(self) -> String;
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
            "0b" => Ok(Self::from_bin_str(&s[2..])?),
            "0o" => Ok(Self::from_oct_str(&s[2..])?),
            "0x" => Ok(Self::from_hex_str(&s[2..])?),
            _ => Err(error::Error::InvalidRadixPrefix),
        }
    }
    
    fn from_bin_str(s: &str) -> Result<Self, error::Error> {
        Ok(s
            .chars()
            .filter(|c| c.is_digit(2))
            .map(|c| c == '1')
            .collect()
        )
    }

    fn from_oct_str(s: &str) -> Result<Self, error::Error> {
        Ok(s
            .chars()
            .filter(|c| c.is_digit(8))
            .flat_map(|c| format!("{:3b}", c.to_digit(8).unwrap())
                .chars()
                .collect::<Vec<_>>()
            )
            .map(|c| c == '1')
            .collect()
        )
    }
    
    fn from_hex_str(s: &str) -> Result<Self, error::Error> {
        Ok(s
            .chars()
            .filter(|c| c.is_digit(16))
            .flat_map(|c| format!("{:4b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
            )
            .map(|c| c == '1')
            .collect()
        )
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
            .map(|b| if b == true { '1' } else { '0' })
            .collect()
    }
    
    fn into_bin_string(self) -> String {
        self
            .into_iter()
            .map(|b| if b == true { '1' } else { '0' })
            .collect()
    }
}
