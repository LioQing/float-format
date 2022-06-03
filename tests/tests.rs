use float_format::*;

#[test]
fn print_test() {
    let float = Float::from_bits(
        Format::ieee_binary32(),
        BitPattern::from_str("0x4640e666").unwrap(),
    ).unwrap();

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", float.as_f64());

    let float = Float::from_fields(
        Format::new(64, 128, 127),
        Some(false),
        "0x8C",
        "0x81CC_CC00_0000_0000_0000_0000_0000_0000",
    ).unwrap();

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", float.as_f64());

    let float = Float::from(12345.6f32);

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", float.as_f64());
}

#[test]
fn ieee_formats() {
    assert_eq!(Format::ieee_binary32(), Format::new(8, 23, 127));
    assert_eq!(Format::ieee_binary64(), Format::new(11, 52, 1023));
}

#[test]
fn prim_float_types() {
    assert_eq!(Float::from(0.2f32).as_f32(), 0.2f32);
    assert_eq!(Float::from(0.2f64).as_f64(), 0.2f64);

    assert_eq!(Float::from(0.2f32).as_f64(), 0.2f32 as f64);
    assert_eq!(Float::from(0.2f64).as_f32(), 0.2f64 as f32);
}

#[test]
fn radices() {
    let bin = Float::from_bits(
        Format::ieee_binary32(),
        BitPattern::from_str("0b0100_0110_0100_0000_1110_0110_0110_0110").unwrap(),
    ).unwrap();
    
    let oct = Float::from_bits(
        Format::ieee_binary32(),
        BitPattern::from_str("0o10620163146").unwrap(),
    ).unwrap();

    let hex = Float::from_bits(
        Format::ieee_binary32(),
        BitPattern::from_str("0x4640e666").unwrap(),
    ).unwrap();
    
    assert_eq!(bin.as_f32(), oct.as_f32());
    assert_eq!(bin.as_f32(), hex.as_f32());
}