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
            ParseError::ParseIntError(i) => format!("{}",i),
            ParseError::ParseCharError(c) => format!("{}",c),
            ParseError::ParseFloatError(f_e) => format!("{}",f_e),
            ParseError::ParseBoolError(b) => format!("{}",b),
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
pub enum Error{
    ParseError(ParseError),
    IoFormatError,
    ArrayLengthError(ArrayLengthError)
}



impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::ParseError(parse) => format!("{}", parse),
            Error::IoFormatError => "IoFormatError in passed io string.".to_string(),
            Error::ArrayLengthError(array_error) => format!("{}", array_error),
        })
    }
}

impl std::error::Error for Error{}

impl From<ParseError> for Error{
    fn from(value: ParseError) -> Self {
        Error::ParseError(value)
    }
}


