mod primitives;
mod arrays;
mod map;

//#[macro_use]
pub extern crate io_deser;
pub use io_deser::*;

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

pub trait IoDeSer{
    //type Type;
    fn to_io_string(&self, tab: u8)->String;
    fn from_io_string(io_input:&mut String)->Self; // Self::Type
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



// TODO: tuples, slices, tuple structs (struct X(T, T2, T3...))
// TODO DONE: vectors, primitives, structs (check better), strings, arrays(check better?)

// potential solution for tuples https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/