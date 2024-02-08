use std::char::ParseCharError;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

macro_rules! from_implementation {
    ($parse: ident) => {
        impl From<$parse> for ParseError{
            fn from(value: $parse) -> Self {
                ParseError::$parse(value)
            }
        }

        impl From<$parse> for Error{
            fn from(value: $parse) -> Self {
                Error::ParseError(ParseError::from(value))
            }
        }
    };
}

#[derive(Debug)]
pub enum ParseError{
    ParseIntError(ParseIntError),
    ParseCharError(ParseCharError),
    ParseFloatError(ParseFloatError),
    ParseBoolError(ParseBoolError),
}


from_implementation!(ParseIntError);
from_implementation!(ParseFloatError);
from_implementation!(ParseCharError);
from_implementation!(ParseBoolError);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",match self {
            ParseError::ParseIntError(i) => i.to_string(),
            ParseError::ParseCharError(c) => c.to_string(),
            ParseError::ParseFloatError(f_e) => f_e.to_string(),
            ParseError::ParseBoolError(b) => b.to_string(),
        })
    }
}

impl std::error::Error for ParseError{}


///////////////
///////////////

#[derive(Debug)]
pub struct ArrayLengthError{
    pub expected_size: usize,
    pub received_size: usize,
}

impl Display for ArrayLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Received array of size {}, but expected {}.", self.received_size, self.expected_size)
    }
}

#[derive(Debug)]
pub struct FieldNotFoundError{
    pub field_name: String,
    struct_name: String,
}

impl Display for FieldNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field '{}' was not found in struct '{}'.", self.field_name, self.struct_name)
    }
}

impl FieldNotFoundError{
    pub fn new(field:String, struct_name:String)->FieldNotFoundError{FieldNotFoundError{ field_name: field, struct_name }}
}

#[derive(Debug)]
pub struct IoFormatError{
    pub io_input:String,
    pub kind: String
}

impl From<IoFormatError> for Error{
    fn from(value: IoFormatError) -> Self {
        Error::IoFormatError(value)
    }
}

impl Display for IoFormatError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error was found in .io formatted string: {}Message: {}",self.io_input, self.kind)
    }
}

#[derive(Debug)]
/// Represents all errors that might occur during deserialization.
pub enum Error{
    /// See [`ParseError`]
    ParseError(ParseError),
    /// See [`IoFormatError`]
    IoFormatError(IoFormatError),
    /// See [`ArrayLengthError`]
    ArrayLengthError(ArrayLengthError),
    /// See [`FieldNotFoundError`]
    FieldNotFoundError(FieldNotFoundError),
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::ParseError(parse) => parse.to_string(),
            Error::IoFormatError(io_error) => io_error.to_string(),
            Error::ArrayLengthError(array_error) => array_error.to_string(),
            Error::FieldNotFoundError(field_error)=> field_error.to_string(),
        })
    }
}

impl std::error::Error for Error{}

impl From<ParseError> for Error{
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
    }
}


impl Error{
    pub fn io_format(io_input: String, kind: String)->Error{
        Error::IoFormatError(IoFormatError{ io_input, kind })
    }

    pub fn field_not_found(field_name:String, struct_name:String)->Error{
        Error::FieldNotFoundError(FieldNotFoundError{ field_name, struct_name })
    }
}

