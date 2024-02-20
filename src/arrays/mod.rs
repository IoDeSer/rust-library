#![allow(dead_code)]

use std::collections::{HashSet, LinkedList, VecDeque, BinaryHeap, BTreeSet};
use std::hash::Hash;
use crate::{delete_tabulator, from_io, IoDeSer};
use crate::errors::*;

macro_rules! create_iterable_impl {
    ($ty:ident $(, $wh : ident)*) => {
		#[automatically_derived]
		impl <T> IoDeSer for $ty<T>
		where T:IoDeSer,
		$(
		T: $wh,
		)*

		{
			#[inline]
			fn to_io_string(&self, tab: u8) -> String {
				format!("|\n{}\n{}|",iterable_ser(&mut self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
			}

			fn from_io_string(io_input: &mut String) -> crate::Result<Self> {
        		if io_input.lines().count()<3 {return Err(Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Input string needs at least 3 lines. Perhaps it is being serialized from wrong type?".to_string() }));}

				let _ = delete_tabulator(io_input)?;
				let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();

				if objects.is_empty(){
					if io_input.is_empty(){
						objects = Vec::new();
					} else{
						objects = vec![io_input];
					}
				}

				objects.iter().map(|o| Ok(from_io!(o.trim().to_string(), T)?)).collect::<crate::Result<Self>>()
			}
		}
	};
}


create_iterable_impl!(HashSet, Eq, Hash);
create_iterable_impl!(BinaryHeap, Ord);
create_iterable_impl!(BTreeSet, Ord);
create_iterable_impl!(LinkedList);
create_iterable_impl!(VecDeque);
create_iterable_impl!(Vec);


/*impl <'a,T:IoDeSer<'a>> IoDeSer<'_> for [T] {
	type Output =[T];

	fn to_io_string(&self, _tab: u8) -> String {
		todo!()
	}

	fn from_io_string(_io_input: &mut String) -> crate::Result<Self::Output>  {
		todo!()
	}
}*/

// arrays
impl <T: IoDeSer, const N: usize> IoDeSer for [T; N]{
    fn to_io_string(&self, tab: u8) -> String {
		format!("|\n{}\n{}|",iterable_ser(&mut self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self>{
		if io_input.lines().count()<3 {return Err(Error::IoFormatError(IoFormatError{ io_input: io_input.to_owned(), kind: "Input string needs at least 3 lines. Perhaps it is being serialized from wrong type?".to_string() }));}

		let _ = delete_tabulator(io_input)?;
		let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();

		if objects.is_empty(){
			if io_input.is_empty(){
				objects = Vec::new();
			} else{
				objects = vec![io_input];
			}
		}

		if &N != &objects.len(){
			return Err(crate::errors::Error::ArrayLengthError(crate::errors::ArrayLengthError{ expected_size: N, received_size: objects.len() }));
		}

		array_init::try_array_init(|index| Ok(from_io!(objects[index].trim().to_string(), T)?))
    }
}

#[inline]
fn iterable_ser<'a,X: IoDeSer+'a, T: Iterator<Item = &'a X>>(obj: T, tab: u8) -> String {
	let mut array_str = String::new();

	for (index, x) in obj.enumerate() {
		if index > 0 {
			array_str += &format!("\n{}+\n", (0..tab + 1).map(|_| "\t").collect::<String>());
		}

		array_str += &(0..tab + 1).map(|_| "\t").collect::<String>();
		array_str += &x.to_io_string(tab + 1);
	}

	array_str
}