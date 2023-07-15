mod primitives;

pub use deser::*;



pub trait IoDeSer<T>{
    fn to_io_string(self)->String;
    fn from_io_string(io_input:String)->T;
}