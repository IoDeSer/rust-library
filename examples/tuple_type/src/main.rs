use iodeser::*;

fn main() {
    let tuple = (-525, 'H', "Str", vec![1, 3, 2, 6, 10], "String".to_string());
    let io_string = to_io!(&tuple);

    assert_eq!(tuple, from_io!(io_string, (i32, char, &str, Vec<i8>, String)).unwrap());
}