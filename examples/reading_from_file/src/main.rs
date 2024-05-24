use iodeser::*;

fn main() {
    let file_content = std::fs::read_to_string("contents.io").unwrap();
    let array = from_io!(file_content, Vec<char>).unwrap();

    assert_eq!(array, vec!['a', 'b','a', '0']);
}
