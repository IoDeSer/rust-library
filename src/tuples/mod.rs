use crate::{delete_tabulator, from_io, IoDeSer};
use std::fmt::Write;

macro_rules! impl_tuple {
    ($t1:tt | $($idx:tt $t:tt),+ | $max:expr) => {
        #[automatically_derived]
        impl <$t1:IoDeSer,$($t:IoDeSer,)+> IoDeSer for ($t1,$($t,)+){


            fn to_io_string(&self, tab: u8) -> String {
                let mut output = "|\n".to_string();
                let tabs = (0..tab).map(|_| "\t").collect::<String>();
                let more_tabs = (0..tab+1).map(|_| "\t").collect::<String>();

                let _ = write!(output, "{}{}", &more_tabs,self.0.to_io_string(tab+1));

                $(

                    let _ = write!(output, "\n{}+",&more_tabs);
                    let _ = write!(output, "\n{}{}",&more_tabs,self.$idx.to_io_string(tab+1));
                )+

                let _ =  write!(output, "\n{}|", &tabs);
                output
            }

            fn from_io_string(io_input: &mut String) -> crate::Result<Self> {
		        if io_input.lines().count()<3 {return Err(crate::errors::Error::IoFormatError(crate::errors::IoFormatError{ io_input: io_input.to_owned(), kind: "Input string needs at least 3 lines. Perhaps it is being serialized from wrong type?".to_string() }));}
		        let _ = delete_tabulator(io_input)?;

                let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();
                if objects.is_empty(){
                    if io_input.is_empty(){
                        objects = Vec::new();
                    } else{
                        objects = vec![io_input];
                    }
                }

                if &$max != &objects.len(){
                    return Err(crate::errors::Error::ArrayLengthError(crate::errors::ArrayLengthError{ expected_size: $max, received_size: objects.len() }));
                }

                Ok((
                    from_io!(objects[0].to_string(), $t1)?,
                    $(
                        from_io!(objects[$idx].to_string(), $t)?,
                    )+
                ))
            }
        }
    };
}

impl_tuple!(T1 | 1 T2 | 2);
impl_tuple!(T1 | 1 T2, 2 T3 | 3);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4 | 4);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5 | 5);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6 | 6);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7 | 7);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8 | 8);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9 | 9);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10 | 10);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11 | 11);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12 | 12);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13 | 13);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14 | 14);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15 | 15);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16 | 16);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17 | 17);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18 | 18);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19 | 19);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20 | 20);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21 | 21);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22 | 22);
impl_tuple!(T1 | 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23 | 23);
