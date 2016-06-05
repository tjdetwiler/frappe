extern crate classfile;

use std::f32;
use std::f64;
use std::fs::File;

use classfile::*;
use classfile::reader::ClassReader;

macro_rules! assert_float_eq {
    ($epsilon:expr, $a:expr, $b:expr) => {{
        assert!(($a - $b).abs() < $epsilon);
    }}
}

#[test]
fn should_constants_class() {
    // Given
    let mut file = File::open("../test-classes/Constants.class").unwrap();

    // When
    let class = ClassReader::new(&mut file).read_class().unwrap();

    // Then
    // Ints
    assert_eq!(
        0xcafebabeu32 as i32,
        get_const_value("INT_VALUE", &class).as_integer());
    assert_eq!(
        2147483647,
        get_const_value("INT_MAX", &class).as_integer());
    assert_eq!(
        -2147483648,
        get_const_value("INT_MIN", &class).as_integer());

    // Longs
    assert_eq!(
        0xdeadc0ffeebabei64,
        get_const_value("LONG_VALUE", &class).as_long());
    assert_eq!(
        9223372036854775807i64,
        get_const_value("LONG_MAX", &class).as_long());
    assert_eq!(
        -9223372036854775808i64,
        get_const_value("LONG_MIN", &class).as_long());

    // Floats
    assert_float_eq!(
        f32::EPSILON,
        3.4028235E38f64 as f32,
        get_const_value("FLOAT_MAX", &class).as_float());
    assert_float_eq!(
        f32::EPSILON,
        1.17549435E-38f64 as f32,
        get_const_value("FLOAT_MIN_NORMAL", &class).as_float());
    assert_float_eq!(
        f32::EPSILON,
        1.4E-45f32,
        get_const_value("FLOAT_MIN", &class).as_float());
    let float = get_const_value("FLOAT_NEGATIVE_INF", &class).as_float();
    assert!(float.is_sign_negative());
    assert!(float.is_infinite());
    let float = get_const_value("FLOAT_POSITIVE_INF", &class).as_float();
    assert!(!float.is_sign_negative());
    assert!(float.is_infinite());
    let float = get_const_value("FLOAT_NAN", &class).as_float();
    assert!(float.is_nan());

    // Doubles
    assert_float_eq!(
        f64::EPSILON,
        1.7976931348623157E308f64,
        get_const_value("DOUBLE_MAX", &class).as_double());
    assert_float_eq!(
        f64::EPSILON,
        2.2250738585072014E-308f64,
        get_const_value("DOUBLE_MIN_NORMAL", &class).as_double());
    assert_float_eq!(
        f64::EPSILON,
        4.9E-324f64,
        get_const_value("DOUBLE_MIN", &class).as_double());
    let double = get_const_value("DOUBLE_NEGATIVE_INF", &class).as_double();
    assert!(double.is_sign_negative());
    assert!(double.is_infinite());
    let double = get_const_value("DOUBLE_POSITIVE_INF", &class).as_double();
    assert!(!double.is_sign_negative());
    assert!(double.is_infinite());
    let double = get_const_value("DOUBLE_NAN", &class).as_double();
    assert!(double.is_nan());

    // String
    let utf8_index = get_const_value("STRING_VALUE", &class).as_string();
    assert_eq!(
        "This is a string constant",
        class.constants[utf8_index].as_utf8());

    // Class
    let object_field = class.find_field("CLASS_VALUE").unwrap();
    let constant_value = object_field.attrs.constant_value();
    assert!(
        !constant_value.is_some(),
        "Object fields should not have a ConstantValue attribute");
}

fn get_const_value<'a>(field_name: &str, class: &'a ClassFile) -> &'a Constant {
    println!("looking up field {}", field_name);
    let field = class.find_field(field_name).unwrap();
    let const_value_index = field.attrs.constant_value().unwrap();
    &class.constants[const_value_index]
}

