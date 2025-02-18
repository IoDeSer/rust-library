use iodeser::*;

// This example demonstrates deserialization from file.

/* File "contents.io" text content:
|
	|a|
	+
	|b|
	+
	|a|
	+
	|0|
|
*/
fn main() {
    // Read whole text from file to String.
    let file_content = std::fs::read_to_string("contents.io").unwrap();

    // Use file content as argument in 'from_io!' macro.
    let array = from_io!(file_content, Vec<char>).unwrap();

    assert_eq!(array, vec!['a', 'b', 'a', '0']);
}
