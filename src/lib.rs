//! # IoDeSer
//! IoDeSer is a open-source project that allows to **de**serialize and **ser**ialize objects into **.io** formatted String.
//!
//! IoDeSer defines traits, methods, macros and implementations for basic Rust types. See this crate's [source code] for additional information.
//!
//!
//! Currently supports the below types:
//! - Vec<T>
//! - HashMap<K, V>
//! - Array [T; N]
//! - Tuples (T1, T2...)
//! - Primitive types (un/signed integer, char, float, boolean)
//! - String
//! - HashSet<T>
//! - BinaryHeap<T>
//! - BTreeSet<T>
//! - LinkedList<T>
//! - VecDeque<T>
//! - HashMap<T, K>
//! - BTreeMap<T, K>
//!
//! ## Status
//! This crate is in alpha status and **should not** be used in production environment.
//!
//! ## Design
//! The main foundation of this project is cross-language compatibility. See the [project account] for more information about other language libraries status, goals and status.
//!
//! ## Examples
//! ```rust
//! use iodeser::*; // required import
//!
//! #[derive(IoDeSer, Debug)] // required macro derive IoDeSer, Debug is not required
//! struct Person<T: IoDeSer> {
//!     #[io_name("Name")]      // optional renaming
//!     pub name: String,
//!     #[io_name("LastName")]  // optional renaming
//!     pub last_name: String,
//!     #[io_name("Age")]       // optional renaming
//!     #[io_order(LAST)]       // optional ordering using FIRST or LAST keyword
//!     pub age: u8,
//!     #[io_name("Address")]   // optional renaming
//!     #[io_order(FIRST)]      // optional ordering using FIRST or LAST keyword
//!     pub address: Vec<Address<T>>,
//! }
//!
//! #[derive(IoDeSer, Debug)] // required macro derive, Debug is not required
//! struct Address<T: IoDeSer> {
//!     #[io_order(3)]          // optional ordering using integer
//!     pub city: String,
//!     #[io_order(1)]          // optional ordering using integer
//!     pub number: T,
//!     #[io_order(2)]          // optional ordering using integer
//!     pub street: String,
//! }
//!
//! fn main() {
//!     let person = Person::<u8> {
//!         name: "John".to_string(),
//!         last_name: "Kowalski".to_string(),
//!         age: 21,
//!         address: vec![
//!             Address::<u8> {
//!                 city: "Warsaw".to_string(),
//!                 number: 65,
//!                 street: "Tęczowa".to_string(),
//!             },
//!             Address::<u8> {
//!                 city: "Hamburg".to_string(),
//!                 number: 220,
//!                 street: "Strasse".to_string(),
//!             }
//!         ],
//!     };
//!
//!     let io_serialization: String = to_io!(&person); // serialization
//!     println!("{}", &io_serialization);
//!
//!    let person_deserialization: Person<u8> = from_io!(io_serialization, Person<u8>).unwrap(); // deserialization
//!     println!("{:?}", &person_deserialization);
//! }
//! ```
//!
//! [source code]: https://github.com/IoDeSer/rust-library
//! [project account]: https://github.com/IoDeSer

//////////////////////////////////////

mod primitives;
mod arrays;
mod map;
mod tuples;
mod errors;
mod others;
mod references;

pub use errors::Error;
pub use io_deser::*;


/// Alias for a `Result` with the error type [`errors::Error`]
pub type Result<T> = std::result::Result<T, errors::Error>;

/// Trait for serializing and deserializing objects into .io formatted String.
///
/// **Should not** be implemented by end user. Instead, use macro [IoDeSer] via the *derive* attribute.
///
/// ## Examples
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
/// 	pub api_key_string: String,
///		pub address: String,
/// }
pub trait IoDeSer{

    /// Serializes *self* into .io file format.
    ///
    /// # Arguments
    ///  * `tab` - Starting number of tabulators in String. At the beginning should be equal to 0.
    fn to_io_string(&self, tab: u8)->String;

    /// Deserializes .io formatted String into Self.
    ///
    /// Returns result with deserialized object or [`errors::Error`].
    ///
    /// # Arguments
    ///  * `io_input` - .io formatted String.
    ///
    /// # Errors
    /// * [`errors::Error::ParseError`] when deserializing primitive into wrong type
    /// * [`errors::Error::IoFormatError`] when passed String `io_input` is in wrong format
    /// * [`errors::Error::ArrayLengthError`] when deserializing array of size X into size Y
    /// * [`errors::Error::FieldNotFoundError`] when field X was found in .io formatted String, but provided struct does not have one (might occur because of wrong naming or ordering, see [`io_order`] and [`io_name`] attributes)
    fn from_io_string(io_input:&mut String)->Result<Self> where Self: Sized; // Self::Type
}


///////////////////
///////////////////
///////////////////

#[inline]
pub(crate) fn delete_tabulator(io_string: &mut String)->Result<()>{
    if !io_string.starts_with('|') || !io_string.ends_with('|') {
        return Err(errors::IoFormatError {
            io_input: io_string.clone(),
            kind: "String lacks vertical bars at the beginning or end".to_string(),
        }.into());
    }

    let mut ret = String::new();
    for line in io_string.lines().filter(|line| line.len() > 1) {
        ret += &format!("{}\n", &line[1..]);
    }

    *io_string = ret.trim().to_string();
    Ok(())
}

#[macro_export]
/// Deserializes .io formatted String into Self.
///
/// Returns result with deserialized object or [`errors::Error`].
///
/// # Arguments
/// * `io_string` - .io formatted String
/// * `type` - type of the deserialized object
///
/// # Errors
/// * [`errors::Error::ParseError`] when deserializing primitive into wrong type
/// * [`errors::Error::IoFormatError`] when passed String `io_input` is in wrong format
/// * [`errors::Error::ArrayLengthError`] when deserializing array of size X into size Y
/// * [`errors::Error::FieldNotFoundError`] when field X was found in .io formatted String, but provided struct does not have one (might occur because of wrong naming or ordering, see [`io_order`] and [`io_name`] attributes)
///
/// # Examples
///
/// ```
/// use iodeser::*;
///
/// let io_string = /* read from string or .io file */ "|\n\n\n|".to_string();
/// let object : Vec<i32> = from_io!(io_string, Vec<i32>).unwrap();
/// ```
macro_rules! from_io{
    ($obj: expr, $type: ty)=>{
        <$type>::from_io_string(&mut $obj.clone())
    };
}

#[macro_export]
/// Serialize this value via reference into .io file format.
///
/// Returns .io formatted String.
///
/// # Arguments
///
/// * `obj` - A reference to an object that implements IoDeSer trait.
///
/// # Examples
///
/// ```
/// use iodeser::*;
///
/// let v = vec![0,23,5,-231,37];
/// let io_string = to_io!(&v);
/// ```
macro_rules! to_io{
    ($obj: expr)=>{
        {
        $obj.to_io_string(0)
            }
    };
}



// TODO: slices, tuple structs (struct X(T, T2, T3...))
// potential solution for tuples https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/