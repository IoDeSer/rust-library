use iodeser::*;

fn main() {
    let integer = 51237;
    let string = "string literal".to_string();
    let str = "borrowed string";

    let io_string_integer = to_io!(&integer);
    let io_string_string = to_io!(&string);
    let io_string_str = to_io!(&str);

    let deserialized_integer = from_io!(io_string_integer, i32).unwrap();
    let deserialized_string = from_io!(io_string_string, String).unwrap();
    let deserialized_str = from_io!(io_string_str, &str).unwrap();

    assert_eq!(deserialized_integer, integer);
    assert_eq!(deserialized_string, string);
    assert_eq!(deserialized_str, str);
}
