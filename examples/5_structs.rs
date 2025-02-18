use iodeser::*;

// This example demonstrates serialization and deserialization of a structs
//  considering 2 (out of 3) types of structs: tuple and with named fields.
// NOTE: Unit struct is NOT supported yet.

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct NamedStruct {
    private_field: char, // Structs with private fields are supported. Their value is not serialized and need to implement Default trait on deserialization.
    pub value: i32,
    pub array: Vec<i32>,
}

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct TupleStruct(pub i32, pub Vec<i32>, bool);

fn main() {
    // initialize objects
    let named = NamedStruct { private_field:'E', value: 0, array: vec![0, 1, 2] };
    let tuple = TupleStruct(3, vec![3, 4, 5], true);

    // serialization of each object
    let io_string_named_struct = to_io!(&named);
    let io_string_tuple_struct = to_io!(&tuple);
    println!("Named:\n{}\n\nTuple:\n{}\n", io_string_named_struct, io_string_tuple_struct);

    // deserialization of each object
    let named_deserialized = from_io!(io_string_named_struct, NamedStruct).unwrap();
    let tuple_deserialized = from_io!(io_string_tuple_struct, TupleStruct).unwrap();

    println!("Named:\nOriginal\t{:?}\nDeserialized\t{:?}\n", named, named_deserialized);
    println!("Tuple:\nOriginal\t{:?}\nDeserialized\t{:?}", tuple, tuple_deserialized);

    /*
        Because NamedStruct has private field, which uses the default method to initialize,
        named != named_deserialized, 
        however named.value and named.array == named_deserialized.value and named_deserialized.array
    
        The same goes for TupleStruct, tuple != tuple_deserialized,
        but tuple.0 and tuple.1 == tuple_deserialized.0 and tuple_deserialized.1
     
        This would fail:
        //assert_eq!(named, from_io!(io_string_named_struct, NamedStruct).unwrap());
        //assert_eq!(tuple, from_io!(io_string_tuple_struct, TupleStruct).unwrap());
    */

    // This works
    assert!(named.value == named_deserialized.value && named.array == named_deserialized.array);
    assert!(tuple.0 == tuple_deserialized.0 && tuple.1 == tuple_deserialized.1);
}
/*Output
Named:
|
        value->|0|
        array->|
                |0|
                +
                |1|
                +
                |2|
        |
|

Tuple:
|
        |3|
        +
        |
                |3|
                +
                |4|
                +
                |5|
        |
|

Named:
Original        NamedStruct { private_field: 'E', value: 0, array: [0, 1, 2] }
Deserialized    NamedStruct { private_field: '\0', value: 0, array: [0, 1, 2] }

Tuple:
Original        TupleStruct(3, [3, 4, 5], true)
Deserialized    TupleStruct(3, [3, 4, 5], false)
*/