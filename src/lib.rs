mod primitives;
mod arrays;

pub use deser::*;

pub struct IoSerialization<'a, T>{
    pub obj: &'a T,
    pub tab: u8,
}

impl<'a, T: IoDeSer> IoSerialization<'a, T> {
    pub fn begin(obj: &'a T)->IoSerialization<'a, T>{
        IoSerialization{ obj, tab: 0 }
    }

    pub fn ser(self)->String{
        self.obj.to_io_string(self.tab)
    }

    pub fn next(obj: &'a T, tab: u8)->IoSerialization<'a, T>{
        IoSerialization{ obj, tab }
    }
}

/*pub fn to_io_string<T: IoDeSer>(obj: &T) -> String{
    IoSerialization::begin(obj).ser()
}*/

pub trait IoDeSer{ //TODO some errors when using generic struct<T> as T
    fn to_io_string(&self, tab: u8)->String;
    fn from_io_string(io_input:&mut String)->Self;
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



// TODO: hashmaps, generics, nested elements in classes (classes and arrays/vectors)
// DONE: vectors, primitives, classes (primitives only), strings, arrays(check better?)