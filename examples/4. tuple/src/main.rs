use iodeser::*;

// This example demonstrates serialization and deserialization of tuples.

fn main() {
    // initialize tuple
    let tuple = (-525, 'H', "Str", vec![1, 3, 2, 6, 10], "String".to_string());

    //serialization
    let io_string = to_io!(&tuple);
    println!("{}", io_string);

    // deserialization
    let tuple_deserialized = from_io!(io_string, (i32, char, &str, Vec<i8>, String)).unwrap();

    // Notice that passing all types is order neccesary.
    // Uncomment the following line to see what happens if deserialization uses the wrong type order:
    // let tuple_wrong = from_io!(io_string, (char, i32, &str, Vec<i8>, String)).unwrap();

    assert_eq!(tuple, tuple_deserialized);
}
/*Output:
|
        |-525|
        +
        |H|
        +
        |Str|
        +
        |
                |1|
                +
                |3|
                +
                |2|
                +
                |6|
                +
                |10|
        |
        +
        |String|
|
*/