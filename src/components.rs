/// Components of a floating point number.
#[derive(Debug)]
pub struct Components {
    /// The sign of the number, `Some(true)` if negative, `None` if the format is unsigned.
    pub neg: Option<bool>,

    /// The exponent bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub exp: String,

    /// The mantissa bit pattern of the number, assumed to be `String` with '1's and '0's.
    pub mant: String,
}