use float_format::Format;
use float_format::IeeeBinary;

#[test]
fn ieee_formats() {
    assert!(matches!(
        Format::ieee_binary32(),
        Format{ signed: true, exp: 8, mant: 23, excess: 127, .. },
    ));
    
    assert!(matches!(
        Format::ieee_binary64(),
        Format{ signed: true, exp: 11, mant: 52, excess: 1023, .. },
    ));
}