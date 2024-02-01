use crate::IoDeSer;
use std::str::FromStr;

macro_rules! impl_iodeser_primitive {
    ($type:ty) => {
        impl IoDeSer<$type> for $type {
            fn to_io_string(self, _tab: u8) -> String {
                format!("|{}|", self)
            }

            fn from_io_string(io_input: &mut String) -> $type {
                let chars: Vec<char> = io_input.chars().collect();
                let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
                <$type>::from_str(&middle_chars).expect("Parse err, TODO")
            }
        }
    };
}

impl IoDeSer<String> for String {
    fn to_io_string(self, _tab: u8) -> String {
        format!("|{}|", self)
    }

    fn from_io_string(io_input: &mut String) -> String {
        let chars: Vec<char> = io_input.chars().collect();
        let middle_chars: String = chars[1..chars.len() - 1].iter().collect();
        middle_chars
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

impl_iodeser_primitive!(char);
