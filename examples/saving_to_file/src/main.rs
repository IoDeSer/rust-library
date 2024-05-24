use iodeser::*;
use std::io::Write;
use std::fs::File;

fn main() {
    let array = vec!['a', 'b','a', '0'];
    let io_string = to_io!(&array);

    let mut file = File::create("contents.io").unwrap();
    file.write_all(io_string.as_bytes());
}
