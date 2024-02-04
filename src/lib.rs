mod primitives;
mod arrays;
mod map;
mod errors;


pub type Result<T> = std::result::Result<T, errors::Error>;

//#[macro_use]
pub extern crate io_deser;
pub use io_deser::*;

pub struct IoSerialization<'a, T>{
    pub obj: &'a T,
    pub tab: u8,
}

impl<'a, T: IoDeSer> IoSerialization<'a, T> {
    pub fn begin(obj: &'a T)->IoSerialization<'a, T>{
        IoSerialization{ obj, tab: 0 }
    }

    pub fn ser(self)->String{
        self.obj.to_io_string(self.tab)
    }

    pub fn next(obj: &'a T, tab: u8)->IoSerialization<'a, T>{
        IoSerialization{ obj, tab }
    }
}

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
    /// Returns deserialized object.
    ///
    /// # Arguments
    ///  * `io_input` - .io formatted String.
    fn from_io_string(io_input:&mut String)->Result<Self> where Self: Sized; // Self::Type
}


///////////////////
///////////////////
///////////////////

pub(crate) fn delete_tabulator(io_string: &mut String){
    let mut ret = String::new();
    let lines: Vec<&str> = io_string.lines().collect();

    for line in lines {
        if line.len() > 1 {
            ret += &format!("{}\n", &line[1..]);
        }
    }

    *io_string = ret.trim().to_string();
}

#[macro_export]
/// Deserialize .io formatted String into an object.
///
/// Returns deserialized object.
///
/// # Arguments
///
/// * `io_string` - .io formatted String
/// * `type` - type of the deserialized object
///
/// # Eaxmples
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
/// # Eaxmples
///
/// ```
/// use iodeser::*;
///
/// let v = vec![0,23,5,-231,37];
/// let io_string = to_io!(&v);
/// ```
macro_rules! to_io{
    ($obj: expr)=>{
        IoSerialization::begin($obj).ser()
    };
}



// TODO: tuples, slices, tuple structs (struct X(T, T2, T3...))
// TODO DONE: vectors, primitives, structs, strings, arrays

// potential solution for tuples https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/