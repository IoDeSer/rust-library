use iodeser::*;

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct MyNamedStruct {
    pub value: i32,
    pub array: Vec<i32>
}

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
struct MyTupleStruct(pub i32, pub Vec<i32>);

fn main() {
    let named = MyNamedStruct{ value: 0, array: vec![0,1,2] };
    let tuple = MyTupleStruct(3, vec![3,4,5]);

    let io_string_named_struct = to_io!(&named);
    let io_string_tuple_struct = to_io!(&tuple);

    assert_eq!(named, from_io!(io_string_named_struct, MyNamedStruct).unwrap());
    assert_eq!(tuple, from_io!(io_string_tuple_struct, MyTupleStruct).unwrap());
}
