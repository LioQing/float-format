use float_format::*;

#[test]
fn print_test() {
    let float = Float::from_comps(
        Format::ieee_binary32(),
        Components::new_bin(
            Some(false),
            format!("{:b}", 128).as_str(),
            format!("{:b}", 1234567).as_str(),
        ).unwrap()
    ).unwrap();

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", float.as_f64());

    let float = Float::from_fields(
        Some(false),
        format!("0b{:0>8b}", 128).as_str(),
        format!("0b{:0>23b}", 1234567).as_str(),
        127,
    ).unwrap();

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", float.as_f64());

    let float = Float::from(2.2943437099456787109375f32);

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