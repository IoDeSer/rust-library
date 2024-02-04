#![allow(dead_code)]

use std::collections::{HashSet, LinkedList, VecDeque, BinaryHeap, BTreeSet};
use std::hash::Hash;
use crate::{delete_tabulator, from_io, IoDeSer};

macro_rules! create_iterable_impl {
    ($ty:ty $(, $wh : ident)*) => {
		impl <T: IoDeSer $(+ $wh)*> IoDeSer for $ty{
			fn to_io_string(&self, tab: u8) -> String {
				format!("|\n{}\n{}|",iterable_ser(&mut self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
			}

			fn from_io_string(io_input: &mut String) -> Self {
				delete_tabulator(io_input);
				let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();

				if objects.is_empty(){
					if io_input.is_empty(){
						objects = Vec::new();
					} else{
						objects = vec![io_input];
					}
				}

				objects.iter().map(|o| from_io!(o.trim().to_string(), T)).collect()
			}
		}
	};
}

create_iterable_impl!(HashSet<T>, Eq, Hash);
create_iterable_impl!(BinaryHeap<T>, Ord);
create_iterable_impl!(BTreeSet<T>, Ord);
create_iterable_impl!(LinkedList<T>);
create_iterable_impl!(VecDeque<T>);
create_iterable_impl!(Vec<T>);

// arrays
impl <T: IoDeSer, const N: usize> IoDeSer for [T; N]{
	//type Type = [T; N];
    fn to_io_string(&self, tab: u8) -> String {
		format!("|\n{}\n{}|",iterable_ser(&mut self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
    }

    fn from_io_string(io_input: &mut String) -> Self {
		delete_tabulator(io_input);
		let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();

		if objects.is_empty(){
			if io_input.is_empty(){
				objects = Vec::new();
			} else{
				objects = vec![io_input];
			}
		}

		array_init::array_init(|index| from_io!(objects[index].trim().to_string(), T))
    }
}

fn iterable_ser<'a, X: IoDeSer + 'a, T: Iterator<Item = &'a X>>(obj: T, tab: u8) -> String {
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