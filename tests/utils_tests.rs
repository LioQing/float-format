use float_format::{
    BitPattern,
    BitPatternExt,
};

#[test]
fn bit_pattern_from_str() {
    let bp = BitPattern::from_value(0b101011001110u32)
        .into_iter()
        .skip_while(|b| !b)
        .collect::<BitPattern>();

    assert_eq!(BitPattern::from_bin_str("101011001110"), bp);
    assert_eq!(BitPattern::from_oct_str("5316"), bp);
    assert_eq!(BitPattern::from_dec_str("2766"), bp);
    assert_eq!(BitPattern::from_hex_str("ace"), bp);
}

#[test]
fn bit_pattern_to_str() {
    let bp = BitPattern::from_bin_str("101011001110");

    assert_eq!(&bp.to_bin_string(), "101011001110");
    assert_eq!(&bp.to_oct_string(), "5316");
    assert_eq!(&bp.to_dec_string(), "2766");
    assert_eq!(&bp.to_hex_string(), "ace");
}