# About
Repository stores code for Rust library that allows to read from and write to .io file format.

### Functions and plans
The current status of both serialization and deserialization:
- [X] Primitive types
- [X] Strings
- [X] Arrays
- [X] Vectors
- [X] Hashmaps
- [X] Structs (with named fields)
- [X] Generics
- [X] Combinations of all above
- [X] Tuples
- [ ] Tuple structs
- [ ] &str type
- [ ] Slices

### Capabilities
 - Serialization of [supported types](#functions-and-plans) using macro **to_io!()** using objects reference,
 - Deserialization of [supported types](#functions-and-plans) using macro **from_io!()** using .io formatted String and wanted objects type,
 - Renaming structs fields in and from .io formatted String using **#[io_name()]** helper macro using String literal as argument.
 - Ordering structs fields in and from .io formatted String using **#[io_order()]** helper macro using either FIRST and LAST keywords or an i16 Integer.

See [example](#example-usage) below for usage of those capabilities.

### Example usage
```rust
use iodeser::*; // required import

#[derive(IoDeSer, Debug)] // required macro derive IoDeSer, Debug is not required
struct Person<T: IoDeSer> {
    #[io_name("Name")]      // optional renaming
    pub name: String,
    #[io_name("LastName")]  // optional renaming
    pub last_name: String,
    #[io_name("Age")]       // optional renaming
    #[io_order(LAST)]       // optional ordering using FIRST or LAST keyword
    pub age: u8,
    #[io_name("Address")]   // optional renaming
    #[io_order(FIRST)]      // optional ordering using FIRST or LAST keyword
    pub address: Vec<Address<T>>,
}

#[derive(IoDeSer, Debug)] // required macro derive, Debug is not required
struct Address<T: IoDeSer> {
    #[io_order(3)]          // optional ordering using integer
    pub city: String,
    #[io_order(1)]          // optional ordering using integer
    pub number: T,
    #[io_order(2)]          // optional ordering using integer
    pub street: String,
}

fn main() {
    let person = Person::<u8> {
        name: "John".to_string(),
        last_name: "Kowalski".to_string(),
        age: 21,
        address: vec![Address::<u8> {
            city: "Warsaw".to_string(),
            number: 65,
            street: "Tęczowa".to_string(),
        }, Address::<u8> {
            city: "Hamburg".to_string(),
            number: 220,
            street: "Strasse".to_string(),
        }],
    };

    let io_serialization: String = to_io!(&person); // serialization by reference
    /* saving to file for example */
    println!("{}", &io_serialization);

    let person_deserialization: Person<u8> = from_io!(io_serialization, Person<u8>); // deserialization
    println!("{:?}", &person_deserialization);
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
        LastName->|Kowalski|
        Age->|21|
|
Person { name: "John", last_name: "Kowalski", age: 21, address: [Address { city: "Warsaw", number: 65, street: "Tęczowa" }, Address { city: "Hamburg", number: 220, street: "Strasse" }] }
*/
```
