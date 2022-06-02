use crate::*;
use bitvec::prelude::*;

pub type BitPattern = BitVec<usize, Msb0>;

pub trait BitPatternExt where Self: Sized {
    fn from_value<T>(val: T) -> Self
    where
        T: BitStore;

    fn from_str(s: &str) -> Result<Self, error::Error>;
    fn from_bin_str(s: &str) -> Result<Self, error::Error>;
    fn from_hex_str(s: &str) -> Result<Self, error::Error>;

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
            "0x" => Ok(Self::from_hex_str(&s[2..])?),
            _ => Err(error::Error::InvalidRadixPrefix),
        }
    }
    
    fn from_bin_str(s: &str) -> Result<Self, error::Error> {
        if s.chars().any(|c| !c.is_digit(2)) {
            return Err(error::Error::InvalidDigit(2));
        }

        Ok(s
            .chars()
            .map(|c| c == '1')
            .collect()
        )
    }
    
    fn from_hex_str(s: &str) -> Result<Self, error::Error> {
        if s.chars().any(|c| !c.is_digit(16)) {
            return Err(error::Error::InvalidDigit(16));
        }

        Ok(s
            .chars()
            .flat_map(|c| format!("{:4b}", c.to_digit(16).unwrap() as u8)
                .chars()
                .collect::<Vec<_>>()
            )
            .map(|c| c == '1')
            .collect()
        )
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
