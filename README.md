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
- [ ] Tuples
- [ ] Tuple structs
- [ ] &str type
- [ ] Slices

### Usage
```rust
use io_de_ser::*; // required import

#[derive(IoDeSer, Debug)] // required macro derive IoDeSer, Debug is not required
struct Person<T : IoDeSer> {
    #[io_name("Name")]		// optional renaming
    pub name: String,
    #[io_name("LastName")]	// optional renaming
    pub last_name: String,
    #[io_name("Age")]		// optional renaming
    pub age: u8,
    #[io_name("Address")]	// optional renaming
    pub address: Address<T>,
}

#[derive(IoDeSer, Debug)] // required macro derive, Debug is not required
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
            street: "Tęczowa".to_string(),
        },
    };

    let io_serialization: String = to_io!(&person); // serialization by reference
    /* saving to file for example */
    println!("{}", &io_serialization);

    let person_deserialization : Person<u8> = from_io!(io_serialization, Person<u8>); // deserialization
    println!("{:?}", &person_deserialization);
}
/*
Output:
|
        Name->|John|
        LastName->|Kowalski|
        Age->|21|
        Address->|
                city->|Warsaw|
                number->|65|
                street->|Tęczowa|
        |
|
Person { name: "John", last_name: "Kowalski", age: 21, address: Address { city: "Warsaw", number: 65, street: "Tęczowa" } }
 */
```