use iodeser::*;

fn main() {
    let array = vec!['a', 'b', 'a', '0'];
    let io_string = to_io!(&array);

    let _ = std::fs::write("contents.io", io_string);
}