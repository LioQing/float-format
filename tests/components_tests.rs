use float_format::Components;

#[test]
fn print_debug() {
    let c = Components::new(
        Some(true),
        "0b101011001110",
        "0b111011010011000",
    ).unwrap();

    assert_eq!(format!("{:?}", c), "Components { sign: -, exp: 101011001110, mant: 111011010011000 }");
    assert_eq!(format!("{:#?}", c), "Components {\n    sign: -,\n    exp: 101011001110,\n    mant: 111011010011000,\n}");
}
