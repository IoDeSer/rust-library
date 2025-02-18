use iodeser::*;

// This example demonstrates serialization and deserialization of basic primitive types.

fn main() {
    // initialization of objects: i32, String and &str
    let integer = -51237;
    let string = "string literal".to_string();
    let str = "borrowed string";

    // serialization
    // macro to_io! need one parameter: reference to the object we want to serialize.
    // it returns correctly formatted String (called io_string)
    let io_string_integer = to_io!(&integer);
    let io_string_string = to_io!(&string);
    let io_string_str = to_io!(&str);

    // printing to console. Notice, that serialized primitive types take up one line.
    println!("i32:\t{}\nString:\t{}\n&str:\t{}", io_string_integer, io_string_string, io_string_str);

    // deserialization. For clarity, now using unwrap() to ignore errors.
    // macro from_io! needs two parameters: string it will deserialize from (called io_string)
    //  and type of the object being deserialized, here: i32 or String or &str .
    // This will be
    let deserialized_integer = from_io!(io_string_integer, i32).unwrap();
    let deserialized_string = from_io!(io_string_string, String).unwrap();
    let deserialized_str = from_io!(io_string_str, &str).unwrap();

    // comparing to original objects
    assert_eq!(deserialized_integer, integer);
    assert_eq!(deserialized_string, string);
    assert_eq!(deserialized_str, str);
}
/*Output:
i32:    |-51237|
String: |string literal|
&str:   |borrowed string|
*/