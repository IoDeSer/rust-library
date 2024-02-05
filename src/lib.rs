mod primitives;
mod arrays;
mod map;
mod tuples;
pub mod errors;

pub extern crate io_deser;
pub use io_deser::*;


/// Alias for a `Result` with the error type [`errors::Error`]
pub type Result<T> = std::result::Result<T, errors::Error>;

/// Trait for serializing and deserializing objects into .io formatted String.
pub trait IoDeSer{
    //type Output;

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

pub(crate) fn delete_tabulator(io_string: &mut String)->Result<()>{
    let test_case = io_string.chars().collect::<Vec<char>>();

    if test_case[0] != '|' ||  test_case[test_case.len() - 1] != '|'{
        return Err(
            errors::IoFormatError{ io_input: io_string.to_owned(),kind: "String lacks vertical bars at the beginning or end".to_string() }.into()
        );
    }

    let mut ret = String::new();
    let lines: Vec<&str> = io_string.lines().collect();

    for line in lines {
        if line.len() > 1 {
            ret += &format!("{}\n", &line[1..]);
        }
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