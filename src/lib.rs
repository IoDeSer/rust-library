mod primitives;
mod arrays;

pub use deser::*;

pub struct IoSerialization<T: IoDeSer<T>>{
    pub obj: T,
    pub tab: u8,
}

impl<T: IoDeSer<T>> IoSerialization<T> {
    pub fn begin(obj: T)->IoSerialization<T>{
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
    fn from_io_string(io_input:&mut String)->T;
}


///////////////////
///////////////////
///////////////////

pub(crate) fn delete_tabulator(io_string: &mut String){
    let mut ret = String::new();
    let lines: Vec<&str> = io_string.lines().collect();

    for line in lines {
        if line.len() > 1 {
            ret += &format!("{}\n", &line[1..]);
        }
    }

    *io_string = ret.trim().to_string();
}

#[macro_export]
macro_rules! from_io{
    ($obj: expr, $type: ty)=>{
        <$type>::from_io_string(&mut $obj.clone())
    };
}

#[macro_export]
macro_rules! to_io{
    ($obj: expr)=>{
        IoSerialization::begin($obj).ser()
    };
}



// TODO: hashmaps, arrays (slices/anything Iterable<>), *whole deserialization*
// DONE: vectors, primitives, classes, strings