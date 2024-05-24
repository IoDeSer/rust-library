use iodeser::*;

#[derive(IoDeSer)]
struct MyGenericStruct<T: IoDeSer> {
    pub value: T,
}

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
enum MyGenericEnum<T1: IoDeSer, T2: IoDeSer> {
    Val1(T1),
    Val2(T2),
    Val3 { a: T1, b: T2 },
}


fn main() {
    let my_struct = MyGenericStruct::<String> { value: "generic value here!".to_string() };
    let io_string = to_io!(&my_struct);

    assert_eq!(my_struct.value, from_io!(io_string,  MyGenericStruct<String>).unwrap().value);

    let my_enum = MyGenericEnum::<i32, &str>::Val3 { a: 1512323, b: "temporary value!" };
    let io_string = to_io!(&my_enum);

    assert_eq!(my_enum, from_io!(io_string,  MyGenericEnum::<i32, &str>).unwrap());
}
