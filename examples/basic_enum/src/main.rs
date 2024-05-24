use iodeser::*;

#[derive(IoDeSer, Debug, PartialEq)] // Debug, PartialEq derives required for assert_eq only
enum MyEnum<'a> {
    UnitValue,
    UnnamedValues(i32, String, char),
    NamedValues { a: u8, b: u8, c: &'a str },
}


fn main() {
    let unit = MyEnum::UnitValue;
    let unnamed = MyEnum::UnnamedValues(54, "test".to_string(), '-');
    let named = MyEnum::NamedValues {
        a: 2,
        b: 123,
        c: "F",
    };

    let io_string_unit_field = to_io!(&unit);
    let io_string_unnamed_field = to_io!(&unnamed);
    let io_string_named_field = to_io!(&named);
    println!("{}\n\n{}\n\n{}", &io_string_unit_field, &io_string_unnamed_field, &io_string_named_field);

    assert_eq!(unit, from_io!(io_string_unit_field, MyEnum).unwrap());
    assert_eq!(unnamed, from_io!(io_string_unnamed_field, MyEnum).unwrap());
    assert_eq!(named, from_io!(io_string_named_field, MyEnum).unwrap());
}
