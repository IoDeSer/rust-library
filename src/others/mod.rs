use crate::{from_io, IoDeSer};

macro_rules! create_reference_impl {
    ($t:ty) => {
        impl <'a, T: IoDeSer> IoDeSer for $t{
            fn to_io_string(&self, tab: u8) -> String {
                (**self).to_io_string(tab) }

            fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
                Ok(Box::leak(Box::new(from_io!(io_input, T)?)))
            }
        }
    };
}

create_reference_impl!(&'a T);
create_reference_impl!(&'a mut T);

impl <T:IoDeSer> IoDeSer for Option<T>{
    fn to_io_string(&self, _tab: u8) -> String {
        match self{
            None => "|||".to_string(),
            Some(val) => val.to_io_string(_tab)
        }
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        if io_input == "|||"{
            Ok(None)
        }else{
            Ok(Some(from_io!(io_input, T)?))
        }
    }
}

impl <T:IoDeSer, E:IoDeSer> IoDeSer for Result<T, E>{
    fn to_io_string(&self, _tab: u8) -> String {
        match self{
            Err(_) => "|||".to_string(),
            Ok(val) => val.to_io_string(_tab)
        }
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        if io_input == "|||"{
            Ok(Err(from_io!(io_input, E)?))
        }else{
            Ok(Ok(from_io!(io_input, T)?))
        }
    }
}