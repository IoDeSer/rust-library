use crate::IoDeSer;
use std::str::FromStr;


// impl< 'a, T: IoDeSer<T>> IoDeSer<&'a [T]> for &'a [T]
// where
// 	&'a T: IoDeSer<&'a T>
// {
// 	fn to_io_string(self, tab: u8) -> String {
// 		format!("|\n{}\n{}|",iterable_ser(self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
// 	}
//
// 	fn from_io_string(io_input: String) -> &'a [T]{
// 		todo!()
// 	}
// }


impl<T: IoDeSer<T>> IoDeSer<Vec<T>> for Vec<T>{
	fn to_io_string(self, tab: u8) -> String {
		format!("|\n{}\n{}|",iterable_ser(self.into_iter(), tab), (0..tab).map(|_| "\t").collect::<String>())
	}

	fn from_io_string(io_input: String) -> Vec<T> {
		todo!()
	}
}

fn iterable_ser<X: IoDeSer<X>,T: Iterator<Item = X>>(obj: T, tab: u8)->String{
	let mut is_first = true;
	let mut array_str = String::new();

	for x in obj {
		if !is_first{
			array_str+=&format!("\n{}+\n",(0..tab+1).map(|_| "\t").collect::<String>());
		}else{
			is_first=false;
		}


		array_str+=&format!("{}{}",(0..tab+1).map(|_| "\t").collect::<String>(),
							x.to_io_string(tab+1));
	}

	array_str
}