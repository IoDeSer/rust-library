#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};
use crate::IoDeSer;

impl <T:IoDeSer, K: IoDeSer> IoDeSer for HashSet<T, K>{
    fn to_io_string(&self, tab: u8) -> String {
        todo!()
    }

    fn from_io_string(io_input: &mut String) -> Self {
        todo!()
    }
}

impl <T:IoDeSer> IoDeSer for BTreeSet<T>{
    fn to_io_string(&self, tab: u8) -> String {
        todo!()
    }

    fn from_io_string(io_input: &mut String) -> Self {
        todo!()
    }
}