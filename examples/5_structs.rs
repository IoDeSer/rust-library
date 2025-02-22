use iodeser::*;

// This example demonstrates serialization and deserialization of a structs
//  considering all types of structs: tuple(), with named fields{} and unit-like.

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct NamedStruct {
    private_field: char, // Structs with private fields are supported. Their value is not serialized and need to implement Default trait on deserialization.
    pub value: i32,
    pub array: Vec<i32>,
}


#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct TupleStruct(pub i32, pub Vec<i32>, bool);


#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct UnitLikeStruct;


fn main() {
    // initialize objects
    let named = NamedStruct { private_field:'E', value: 0, array: vec![0, 1, 2] };
    let tuple = TupleStruct(3, vec![3, 4, 5], true);
    let unit = UnitLikeStruct;

    // serialization of each object
    let io_string_named_struct = to_io!(&named);
    let io_string_tuple_struct = to_io!(&tuple);
    let io_string_unit_struct = to_io!(&unit);
    println!("Named:\n{}\n\nTuple:\n{}\n\nUnit-like:\n{}\n\n=====\n\n", io_string_named_struct, io_string_tuple_struct,io_string_unit_struct);

    // deserialization of each object
    let named_deserialized = from_io!(io_string_named_struct, NamedStruct).unwrap();
    let tuple_deserialized = from_io!(io_string_tuple_struct, TupleStruct).unwrap();
    let unit_deserialized = from_io!(io_string_unit_struct, UnitLikeStruct).unwrap();

    println!("Named:\nOriginal\t{:?}\nDeserialized\t{:?}\n", named, named_deserialized);
    println!("Tuple:\nOriginal\t{:?}\nDeserialized\t{:?}\n", tuple, tuple_deserialized);
    println!("Unit-like:\nOriginal\t{:?}\nDeserialized\t{:?}", unit, unit_deserialized);

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
    assert_eq!(unit, unit_deserialized);
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