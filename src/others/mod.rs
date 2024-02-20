use crate::{from_io, IoDeSer};

impl <'a, T: IoDeSer> IoDeSer for &'a T{
    fn to_io_string(&self, tab: u8) -> String {
        (**self).to_io_string(tab) }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Box::leak(Box::new(from_io!(io_input, T)?)))
    }
}

impl <'a, T: IoDeSer> IoDeSer for &'a mut T{
    fn to_io_string(&self, tab: u8) -> String { (**self).to_io_string(tab) }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Box::leak(Box::new(from_io!(io_input, T)?)))
    }
}