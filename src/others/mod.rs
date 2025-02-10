use std::fmt::Write;

use crate::errors::IoFormatError;
use crate::{from_io, IoDeSer};

macro_rules! create_reference_impl {
    ($t:ty) => {
        impl<'a, T: IoDeSer> IoDeSer for $t {
            fn to_io_string(&self, tab: u8, buffer: &mut String) {
                (**self).to_io_string(tab, buffer);
            }

            fn from_io_string(io_input: &mut String) -> crate::Result<Self>
            where
                Self: Sized,
            {
                Ok(Box::leak(Box::new(from_io!(io_input, T)?)))
            }
        }
    };
}

create_reference_impl!(&'a T);
create_reference_impl!(&'a mut T);

impl<T: IoDeSer> IoDeSer for Option<T> {
    fn to_io_string(&self, tab: u8, buffer: &mut String) {
        match self {
            None => {let _ = write!(buffer, "|||");},
            Some(val) => val.to_io_string(tab, buffer), // todo: apply to buffer
        };
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if io_input == "|||" {
            Ok(None)
        } else {
            Ok(Some(from_io!(io_input, T)?))
        }
    }
}
impl IoDeSer for () {
    fn to_io_string(&self, _tab: u8, buffer: &mut String) {
        let _ = write!(buffer, "|||");
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if io_input == "|||" {
            Ok(())
        } else {
            Err(IoFormatError {
                io_input: io_input.to_string(),
                kind: "There was an error in deserializing '()' type".to_string(),
            }
            .into()) //todo better error message
        }
    }
}

impl<T: IoDeSer, E: IoDeSer> IoDeSer for Result<T, E> {
    fn to_io_string(&self, tab: u8, buffer: &mut String) {
        match self {
            Err(_) => {let _ = write!(buffer, "|||");},
            Ok(val) => val.to_io_string(tab, buffer), // todo: apply to buffer
        };
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if io_input == "|||" {
            Ok(Err(from_io!(io_input, E)?))
        } else {
            Ok(Ok(from_io!(io_input, T)?))
        }
    }
}