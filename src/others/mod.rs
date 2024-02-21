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