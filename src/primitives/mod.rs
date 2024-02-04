use crate::{IoDeSer, Result};
use std::str::FromStr;

macro_rules! impl_iodeser_primitive {
    ($type:ty, $err:ident) => {
        impl IoDeSer for $type {

            fn to_io_string(&self, _tab: u8) -> String {
                format!("|{}|", self)
            }

            fn from_io_string(io_input: &mut String) -> Result<Self> {
                if io_input.len() < 2 {return Err(crate::errors::Error::IoFormatError);}

                let chars: Vec<char> = io_input.chars().collect();
                let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
<<<<<<< Updated upstream
                <$type>::from_str(&middle_chars).expect(&format!("Can't parse value '{}' into type {}", &middle_chars, stringify!($type)))
=======
                Ok(<$type>::from_str(&middle_chars)?)
>>>>>>> Stashed changes
            }
        }
    };
}

impl IoDeSer for String {
    fn to_io_string(&self, _tab: u8) -> String  {
        format!("|{}|", self)
    }

    fn from_io_string(io_input: &mut String) -> Result<Self>  {
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
        Ok(middle_chars)
    }
}


/*impl IoDeSer for str {
    type Type = String;
    fn to_io_string(&self, _tab: u8) -> String {
        format!("|{}|", self)
    }

    fn from_io_string(io_input: &mut String) -> Self::Type {
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
        middle_chars
    }
}

impl IoDeSer for &str {
    type Type = String;
    fn to_io_string(&self, _tab: u8) -> String {
        format!("|{}|", self)
    }

    fn from_io_string(io_input: &mut String) -> Self::Type {
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
        middle_chars
    }
}*/


impl_iodeser_primitive!(i8, ParseIntError);
impl_iodeser_primitive!(i16, ParseIntError);
impl_iodeser_primitive!(i32, ParseIntError);
impl_iodeser_primitive!(i64, ParseIntError);
impl_iodeser_primitive!(i128, ParseIntError);
impl_iodeser_primitive!(isize, ParseIntError);

impl_iodeser_primitive!(u8, ParseIntError);
impl_iodeser_primitive!(u16, ParseIntError);
impl_iodeser_primitive!(u32, ParseIntError);
impl_iodeser_primitive!(u64, ParseIntError);
impl_iodeser_primitive!(u128, ParseIntError);
impl_iodeser_primitive!(usize, ParseIntError);

impl_iodeser_primitive!(f32, ParseFloatError);
impl_iodeser_primitive!(f64, ParseFloatError);

impl_iodeser_primitive!(bool, ParseBoolError);

impl_iodeser_primitive!(char, ParseCharError);
