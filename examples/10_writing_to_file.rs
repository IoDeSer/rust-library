use iodeser::*;

// This example demonstrates serialization to file.

fn main() {
    let array = vec!['a', 'b', 'a', '0'];
    let io_string = to_io!(&array);

    let _ = std::fs::write("contents.io", &io_string);

    assert_eq!(io_string, 
"|
	|a|
	+
	|b|
	+
	|a|
	+
	|0|
|");
}