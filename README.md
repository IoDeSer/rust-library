# About
Repository stores code for Rust library that allows to read from and write to .io file format.

### Functions and plans
- [X] Primitive types
- [X] Strings
- [X] Arrays
- [X] Vectors
- [X] Hashmaps
- [X] Classes (with named fields)
- [X] Generics
- [X] Combinations of all above
- [ ] Tuples
- [ ] Tuple structs
- [ ] &str type
- [ ] Slices

### Usage
```rust
use io_de_ser::*; // required import

#[derive(IoDeSer)] // required macro derive IoDeSer, PartialEq is not required
struct Person<T : IoDeSer> {
    pub name: String,
    pub last_name: String,
    pub age: u8,
    pub address: Address<T>,
}

#[derive(IoDeSer)] // required macro derive IoDeSer, PartialEq is not required
struct Address<T : IoDeSer> {
    pub city: String,
    pub number: T,
    pub street: String,
}

fn main() {
    let person = Person::<u8>{
        name: "John".to_string(),
        last_name: "Kowalski".to_string(),
        age: 21,
        address: Address::<u8> {
            city: "Warsaw".to_string(),
            number: 65,
            street: "".to_string(),
        },
    };

    let io_serialization: String = to_io!(&person); // serialization by reference
    /* saving to file for example */

    let person_deserialization : Person<u8> = from_io!(io_serialization, Person<u8>); // deserialization
}
```