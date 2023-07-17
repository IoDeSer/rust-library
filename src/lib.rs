mod primitives;
mod arrays;

pub use deser::*;

pub struct IoSerialization<T: IoDeSer<T>>{
    pub obj: T,
    pub tab: u8,
}

impl<T: IoDeSer<T>> IoSerialization<T> {
    fn begin(obj: T)->IoSerialization<T>{
        IoSerialization{ obj, tab: 0 }
    }

    pub fn ser(self)->String{
        self.obj.to_io_string(self.tab)
    }

    pub fn next(obj: T, tab: u8)->IoSerialization<T>{
        IoSerialization{ obj, tab }
    }
}

pub fn to_io_string<T: IoDeSer<T>>(obj: T) -> String{
    let ser = IoSerialization::begin(obj);
    ser.ser()
}

pub trait IoDeSer<T>{ //TODO some errors when using generic struct<T> as T
    fn to_io_string(self, tab: u8)->String;
    fn from_io_string(io_input:String)->T;
}

// TODO: vectors, hashmaps, arrays, whole deserialization

/*
EXAMPLE:

use io_de_ser::*;

#[derive(IoDeSer, Debug)]
struct Person {
	pub name: String,
	pub age: i32,
	pub test: Test,
}

#[derive(IoDeSer, Debug)]
struct Test {
	pub year: u64,
	pub test2: Test2,
}

#[derive(IoDeSer, Debug)]
struct Test2 {
	pub char_eg: char,
}


fn main() {
	let x = Person { name: "example_name".to_string(), age: 1, test: Test { year: 2023, test2: Test2 { char_eg: 'z' } } };
	let f = to_io_string(x);
	println!("{f}");
}

OUTPUT:
|
        name->|example_name|
        age->|1|
        test->|
                year->|2023|
                test2->|
                        char_eg->|z|
                |
        |
|
 */