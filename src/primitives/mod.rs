use crate::{IoDeSer, Result};
use std::str::FromStr;
use crate::errors::IoFormatError;

fn escape<T: ToString>(s: &T) -> String {
    s.to_string().escape_debug().to_string().replace("|", "||")
}


pub fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        match chars.next() {
            Some('b') => result.push('\u{0008}'),
            Some('f') => result.push('\u{000C}'),
            Some('n') => result.push('\n'),
            Some('r') => result.push('\r'),
            Some('t') => result.push('\t'),
            Some('\'') => result.push('\''),
            Some('\"') => result.push('\"'),
            Some('\\') => result.push('\\'),
            Some('0') => result.push('\0'),
            _ => {},
        };
    }

    result.replace("||", "|")
}



macro_rules! impl_iodeser_primitive {
    ($type:ty) => {
        #[automatically_derived]
        impl IoDeSer for $type {


            #[inline]
            fn to_io_string(&self, _tab: u8) -> String {
                format!("|{}|", self)
            }

            fn from_io_string(io_input: &mut String) -> Result<Self> {
                let chrs = io_input.chars().collect::<Vec<char>>();
                if chrs[0] != '|' ||  chrs[chrs.len() - 1] != '|'{
                    return Err(
                        crate::errors::IoFormatError{ io_input: io_input.to_owned(),kind: "String lacks vertical bars at the beginning or end".to_string() }.into()
                    );
                }
                if chrs.len() < 3 {return Err(crate::errors::Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Empty input".to_string() }));}
                let middle_chars: String = chrs[1..chrs.len() - 1].iter().collect();

                Ok(<$type>::from_str(&middle_chars)?)
            }
        }
    };
}

#[automatically_derived]
impl IoDeSer for String {
    #[inline]
    fn to_io_string(&self, _tab: u8) -> String  {
        format!("|{}|", escape(self))
    }

    fn from_io_string(io_input: &mut String) -> Result<Self>  {
        if io_input.len() < 2 {return Err(crate::errors::Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Input was too short. Perhaps it lacks vertical bar characters '|'?".to_string() }));}
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
        Ok(unescape(&middle_chars))
    }
}

#[automatically_derived]
impl <'a> IoDeSer for &'a str
{
    #[inline]
    fn to_io_string(&self, _tab: u8) -> String  {
        format!("|{}|", escape(&self))
    }

    fn from_io_string(io_input: &mut String) -> Result<Self> {
        if io_input.len() < 2 {return Err(crate::errors::Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Input was too short. Perhaps it lacks vertical bar characters '|'?".to_string() }));}
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars:String = unescape(&chars[1..chars.len() - 1].iter().collect::<String>());
        Ok(Box::leak(middle_chars.into_boxed_str()))
    }
}

#[automatically_derived]
impl IoDeSer for char
{
    #[inline]
    fn to_io_string(&self, _tab: u8) -> String  {
        format!("|{}|", escape(&self))
    }

    fn from_io_string(io_input: &mut String) -> Result<Self> {
        if io_input.len() < 2 {return Err(crate::errors::Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Input was too short. Perhaps it lacks vertical bar characters '|'?".to_string() }));}
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars:String = unescape(&chars[1..chars.len() - 1].iter().collect::<String>());
        Ok(char::from_str(&middle_chars).expect("UNHANDLED ERROR. CHAR FROM STR FAILED"))
    }
}


impl_iodeser_primitive!(i8);
impl_iodeser_primitive!(i16);
impl_iodeser_primitive!(i32);
impl_iodeser_primitive!(i64);
impl_iodeser_primitive!(i128);
impl_iodeser_primitive!(isize);

impl_iodeser_primitive!(u8);
impl_iodeser_primitive!(u16);
impl_iodeser_primitive!(u32);
impl_iodeser_primitive!(u64);
impl_iodeser_primitive!(u128);
impl_iodeser_primitive!(usize);

impl_iodeser_primitive!(f32);
impl_iodeser_primitive!(f64);

impl_iodeser_primitive!(bool);
