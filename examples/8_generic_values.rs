use iodeser::*;

// This example demonstrates serialization and deserialization of objects with generic values.

#[derive(IoDeSer, PartialEq, Debug)]
struct MyGenericStruct<T1: IoDeSer, T2: IoDeSer> // all generic MUST implement IoDeSer trait
{
    pub value: T1,
    pub value2: T2
}

#[derive(IoDeSer, PartialEq, Debug)]
struct MyStruct{
    pub value: String,
    pub value2: u8
}

fn main() {
    // Initialize objects
    let generic_struct = MyGenericStruct{ value: vec![1.0, 5.0, -4583.0, 55.0, 123.0, 33333.0, -3431.0, 934.5], 
                                                            value2: MyStruct{value: "value".into(), value2:0}
                                                        };

    // Serialization
    let serialized = to_io!(&generic_struct);
    println!("{}", serialized);

    // Deserialization: if structure includes generic, second parameter in 'from_io!'
    //  must include all of their types. 
    let deserialized = from_io!(serialized, MyGenericStruct<Vec<f32>, MyStruct>).unwrap();

    assert_eq!(generic_struct, deserialized);
}
/*Output:
|
        value->|
                |1|
                +
                |5|
                +
                |-4583|
                +
                |55|
                +
                |123|
                +
                |33333|
                +
                |-3431|
                +
                |934.5|
        |
        value2->|
                value->|value|
                value2->|0|
        |
|
*/