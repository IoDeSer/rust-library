# Introduction
Repository stores code for Rust library that allows to read from and write to .io file format.

### About
.io file is a text format that contains serialized object. Each object is stored between two vertical bars *|X|* in one line for primitive object or more, for iterables and structures. It is build to be cross-platform, language-independent and human-readable.

You can read more about .io de/serialization file format goal and mission [here](https://github.com/IoDeSer).

### Functions and plans
The current status of both serialization and deserialization:
- [X] Primitive types
- [X] Strings
- [X] Arrays
- [X] Vectors
- [X] Hashmaps
- [X] Structs (Named{} and Tuple() and Unit-like)
- [X] Generic objects (<T>)
- [X] Tuples
- [X] &str type
- [X] Slices
- [X] Option
- [X] Result
- [X] Combinations of all above
- [X] Enums (Unit, Unnamed(), Named{})
- [X] Reference types (&T)
- [ ] Unit type `()`

Full list of supported types can be found in this [crate's documentation](https://docs.rs/iodeser/latest/iodeser/trait.IoDeSer.html#foreign-impls).

## Capabilities
 - Serialization of [supported types](#functions-and-plans) using macro **to_io!()** using objects reference,
 - Deserialization of [supported types](#functions-and-plans) using macro **from_io!()** using .io formatted String and objects type,
 - Renaming structs fields in and from .io formatted String using **#[io_name()]** helper macro using String literal as argument.
 - Ordering structs fields in and from .io formatted String using **#[io_order()]** helper macro using either FIRST and LAST keywords or an i16 Integer.
 - Ignoring public fields in de/serialization using **#[io_ignore]** helper macro
 - Allowing to de/serialize private fields using **#[io_allow]** helper macro.

Refer to [example](#example-usage) to see, how these capabilities can be utilized and how they affect the serialized string.

### How to use
First, you need to import iodeser crate. Inside of **Cargo.toml** add:
```toml
[dependencies]
# ...
iodeser = "0.6.0"
```

Next, you need to import crate in a code:
```rust
use iodeser::*;
```

To serialize or deserialize ready objects use [macros](#capabilities). Remember about passing desired objects type to deserialization macro.
```rust
// serialize
let number: i32 = 37;
let serialized_number = to_io!(number);

//deserialize
let deserialized_number = from_io!(serialized_number, i32).unwrap();

assert_eq(number, deserialized_number);
```

To de/serialize created structs or enums you need to use *derive* trait *IoDeSer*:
```rust
#[derive(IoDeSer)]
struct Animal{
    pub name: String
}

let animal = Animal(name: "Cat".into());
let serialized_animal = to_io!(animal);
let deserialized_animal = from_io!(serialized_animal, Animal);
```

### Example usage
```rust
use iodeser::*; // required import

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive IoDeSer, Debug and PartialEq is not required
struct Person<'a> {
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
    pub address: Vec<Address<'a>>,
}

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
struct Address<'a> {
    #[io_order(3)]          // optional ordering using integer
    pub city: &'a str,
    #[io_order(1)]          // optional ordering using integer
    pub number: AddressNumberType<'a>,
    #[io_order(2)]          // optional ordering using integer
    pub street: &'a str,
}

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
enum AddressNumberType<'a>{
    Numeric(u16),
    String(&'a str)
}

fn main() {
    let person = Person {
        name: "John",
        second_name: None,
        last_name: "Kowalski",
        age: 21,
        address: vec![
            Address {
                city: "Warsaw",
                number: AddressNumberType::Numeric(65),
                street: "Tęczowa",
            },
            Address {
                city: "Hamburg",
                number: AddressNumberType::String("220a"),
                street: "Strasse",
            },
        ],
    };

    let io_serialization: String = to_io!(&person); // serialization
    println!("{}", &io_serialization);

    let person_deserialization: Person = from_io!(io_serialization, Person).unwrap(); // deserialization
    println!("{:?}", &person_deserialization);

    assert_eq!(person, person_deserialization);
}
/*
Output:
|
	Address->|
		|
			number->|
				Numeric->|
					|65|
				|
			|
			street->|Tęczowa|
			city->|Warsaw|
		|
		+
		|
			number->|
				String->|
					|220a|
				|
			|
			street->|Strasse|
			city->|Hamburg|
		|
	|
	Name->|John|
	SecondName->|||
	LastName->|Kowalski|
	Age->|21|
|
Person { name: "John", second_name: None, last_name: "Kowalski", age: 21, address: [Address { city: "Warsaw", number: Numeric(65), street: "Tęczowa" }, Address { city: "Hamburg", number: String("220a"), street: "Strasse" }] }
*/
```

See more examples on GitHub in [examples](https://github.com/IoDeSer/rust-library/tree/main/examples).

## Features
Optional features for now only include crate [chrono](https://docs.rs/chrono/latest/chrono/).

To use it you need to turn on feature in *Cargo.toml*:
```toml
[dependencies]
# ...
iodeser = {version = "0.6.0", features = ["chrono"]}
```

You can either inclide *chrono* as a dependency in *Cargo.toml* or use it from *iodeser* package:
```rust
use iodeser::chrono::*;
```
