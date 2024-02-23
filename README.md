# About
Repository stores code for Rust library that allows to read from and write to .io file format.

### Functions and plans
The current status of both serialization and deserialization:
- [X] Primitive types
- [X] Strings
- [X] Arrays
- [X] Vectors
- [X] Hashmaps
- [X] Structs (Named{} and tuple())
- [X] Generics
- [X] Tuples
- [X] &str type
- [X] Slices
- [X] Option
- [X] Result
- [X] Combinations of all above
- [ ] Enums

Full list of supported types can be found in this [crate's documentation](https://docs.rs/iodeser/latest/iodeser/trait.IoDeSer.html#foreign-impls).

### Capabilities
 - Serialization of [supported types](#functions-and-plans) using macro **to_io!()** using objects reference,
 - Deserialization of [supported types](#functions-and-plans) using macro **from_io!()** using .io formatted String and wanted objects type,
 - Renaming structs fields in and from .io formatted String using **#[io_name()]** helper macro using String literal as argument.
 - Ordering structs fields in and from .io formatted String using **#[io_order()]** helper macro using either FIRST and LAST keywords or an i16 Integer.

See [example](#example-usage) below for usage of those capabilities.

### Example usage
```rust
use iodeser::*; // required import

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive IoDeSer, Debug and PartialEq is not required
struct Person<'a, T: IoDeSer> {
    #[io_name("Name")]      // optional renaming
    pub name: &'a str,
    #[io_name("SecondName")]  // optional renaming
    pub second_name: Option<&'a str>,
    #[io_name("LastName")]  // optional renaming
    pub last_name: &'a str,
    #[io_name("Age")]       // optional renaming
    #[io_order(LAST)]       // optional ordering using FIRST or LAST keyword
    pub age: u8,
    #[io_name("Address")]   // optional renaming
    #[io_order(FIRST)]      // optional ordering using FIRST or LAST keyword
    pub address: Vec<Address<'a, T>>,
}

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
struct Address<'a, T: IoDeSer> {
    #[io_order(3)]          // optional ordering using integer
    pub city: &'a str,
    #[io_order(1)]          // optional ordering using integer
    pub number: T,
    #[io_order(2)]          // optional ordering using integer
    pub street: &'a str,
}

fn main() {
    let person = Person::<u8> {
        name: "John",
        second_name: None,
        last_name: "Kowalski",
        age: 21,
        address: vec![
            Address::<u8> {
                city: "Warsaw",
                number: 65,
                street: "Tęczowa",
            },
            Address::<u8> {
                city: "Hamburg",
                number: 220,
                street: "Strasse",
            },
        ],
    };

    let io_serialization: String = to_io!(&person); // serialization
    println!("{}", &io_serialization);

    let person_deserialization: Person<u8> = from_io!(io_serialization, Person<u8>).unwrap(); // deserialization
    println!("{:?}", &person_deserialization);

    assert_eq!(person, person_deserialization);
}
/*
Output:
|
        Address->|
                |
                        number->|65|     
                        street->|Tęczowa|
                        city->|Warsaw|   
                |
                +
                |
                        number->|220|    
                        street->|Strasse|
                        city->|Hamburg|
                |
        |
        Name->|John|
        SecondName->|||
        LastName->|Kowalski|
        Age->|21|
|
Person { name: "John", second_name: None, last_name: "Kowalski", age: 21, address: [Address { city: "Warsaw", number: 65, street: "Tęczowa" }, Address { city: "Hamburg", number: 220, street: "Strasse" }] }
*/
```
