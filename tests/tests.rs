use float_format::*;

#[test]
fn print_test() {
    let float = Float::from_str(Format::ieee_binary32(), "123456").unwrap();

    println!("{}", float);

    println!("{}", 123456f32);
    println!("{:b}", 123456f32.to_bits());
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

#[test]
fn parse_and_display() {
    assert_eq!(
        format!("{:.15}", Float::from_str(Format::ieee_binary32(), "0.000000000000123456789").unwrap()),
        format!("{:.15}", 0.000000000000123456789f32),
    );

    assert_eq!(
        format!("{}", Float::from_str(Format::ieee_binary32(), "123456789876543212345678987654321234567").unwrap())[..6],
        format!("{}", 123456789876543212345678987654321234567f32)[..6],
    );

    assert_eq!(
        format!("{:.3}", Float::from_str(Format::ieee_binary32(), "12345.6789").unwrap()),
        format!("{:.3}", 12345.6789f32),
    );

    assert_eq!(
        format!("{:.6}", Float::from_str(Format::ieee_binary32(), "0.123456").unwrap()),
        format!("{:.6}", 0.123456f32),
    );

    assert_eq!(
        format!("{}", Float::from_str(Format::ieee_binary32(), "123456").unwrap()),
        format!("{}", 123456f32),
    );
}