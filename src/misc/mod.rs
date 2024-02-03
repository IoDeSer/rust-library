#![allow(dead_code)]

use std::collections::BinaryHeap;
use crate::IoDeSer;

impl <T:IoDeSer> IoDeSer for BinaryHeap<T>{
    fn to_io_string(&self, tab: u8) -> String {
        todo!()
    }

    fn from_io_string(io_input: &mut String) -> Self {
        todo!()
    }
}