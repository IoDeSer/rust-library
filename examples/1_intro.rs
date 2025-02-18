// This example serves as an introduction as an introduction to serialization and deserialization
// using iodeser crate.

// Serialization works using macro to_io!(OBJECT)
// Deserialization works using macro from_io!(STRING, TYPE)
// To use them, we need to import them from iodeser crate:
use iodeser::{to_io, from_io};

/* Example:
    let x = 5;
    let serialized = to_io!(x);
    let deserialized = from_io!(serialized, i32).unwrap();
    assert_eq!(x, deserialized);
*/

// Those macros can be used out of the box with most build-in types.
// However if we need to de/serialize created struct/enum they need to implement IoDeSer trait.
// This trait *SHOULD NOT* be implemented by end user, rather by using derive macro with the same name.
// It also need to be imported:
use iodeser::IoDeSer;

/* Example:
    #[derive(IoDeSer)]
    struct MyStruct(pub i32);
*/

// For convienience, we can also import all 3 using:
// use iodeser::*;

fn main() {
    // initializing object
    let x: i32 = 5;

    // serializing using to_io! macro with one argument: object to serialize
    let serialized: String = to_io!(x);

    // deserializing using from_io! macro with two arguments:
    //  formatted string with serialized object,
    //  type of object we want to deserialize.
    // normally, this macro returns Result, but for convienience we can ignore this using 'unwrap()'.
    // in production, this should be handled using 'match' statement.
    let deserialized: i32 = from_io!(serialized, i32).unwrap();

    // compare to original
    assert_eq!(x, deserialized);
}
