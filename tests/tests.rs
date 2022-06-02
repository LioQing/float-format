use float_format::*;

#[test]
fn print_test() {
    let float = Float::from_comps(
        Format::ieee_binary32(),
        Components {
            neg: Some(false),
            exp: format!("{:b}", 128),
            mant: format!("{:b}", 0),
        }
    ).unwrap();

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", Into::<f64>::into(float));

    let float = Float::from(2f32);

    println!("{:?}", float);
    println!("{:?}", float.to_comps());
    println!("{:.64}", Into::<f64>::into(float));
}

#[test]
fn ieee_formats() {
    assert_eq!(Format::ieee_binary32(), Format::new(8, 23, 127));
    assert_eq!(Format::ieee_binary64(), Format::new(11, 52, 1023));
}

#[test]
fn prim_float_types() {
    assert_eq!(Into::<f32>::into(Float::from(0.2f32)), 0.2f32);
    assert_eq!(Into::<f64>::into(Float::from(0.2f64)), 0.2f64);

    assert_eq!(Into::<f64>::into(Float::from(0.2f32)), 0.2f32 as f64);
    assert_eq!(Into::<f32>::into(Float::from(0.2f64)), 0.2f64 as f32);
}