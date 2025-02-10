#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use crate::{delete_tabulator, from_io, IoDeSer};

macro_rules! map_to_io {
    ($s:expr, $tab: expr, $buffer: ident) => {
        {
            let _ = write!($buffer, "|\n");
            let mut first_iteration = true;
            let tabs = (0.. $tab + 1).map(|_| "\t").collect::<String>();


            for (k, v) in $s {
                if !first_iteration{
                    let _ = write!($buffer, "\n{}+\n", &tabs);
                }

                let _ = write!($buffer, "{}", &tabs);
                handle_hashmap_iteration(k,v,  $tab+1, $buffer);

                if first_iteration{
                    first_iteration=false;
                }
            }
            let _ = write!($buffer, "\n{}|", (0.. $tab).map(|_| "\t").collect::<String>());
        }
    };
}
impl <T:IoDeSer + Ord, K:IoDeSer> IoDeSer for BTreeMap<T, K>
where T : Ord
{
    #[inline]
    fn to_io_string(&self, tab: u8, buffer: &mut String) {
        map_to_io!(self, tab, buffer);
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> {
        let _ = delete_tabulator(io_input)?;
        let mut objects = io_input.split_terminator("\n+\n").collect::<Vec<&str>>();
        if objects.is_empty(){
            if io_input.is_empty(){
                objects = Vec::new();
            } else{
                objects = vec![io_input];
            }
        }

        objects
            .iter_mut()
            .map(|o| handle_hashmap_from_io_iteration::<T,K>(&mut o.to_string()))
            .collect::<crate::Result<Self>>()
    }
}

impl<T: IoDeSer + Eq+ PartialEq+Hash,K: IoDeSer> IoDeSer for HashMap<T, K>
    where T : Eq +Hash
{

    #[inline]
    fn to_io_string(&self, tab: u8, buffer: &mut String) {
        map_to_io!(self, tab, buffer);
    }

    fn from_io_string(io_input: &mut String) ->  crate::Result<Self> {
        let _  = delete_tabulator(io_input)?;
        let mut objects = io_input.split_terminator("\n+\n").collect::<Vec<&str>>();
        if objects.is_empty(){
            if io_input.is_empty(){
                objects = Vec::new();
            } else{
                objects = vec![io_input];
            }
        }

        objects
            .iter_mut()
            .map(|o| handle_hashmap_from_io_iteration::<T,K>(&mut o.to_string()))
            .collect::<crate::Result<Self> >()
    }
}

#[inline]
fn handle_hashmap_from_io_iteration<T: IoDeSer, K: IoDeSer>(io_string: &mut String)->crate::Result<(T,K)>{
    let _ = delete_tabulator(io_string)?;
    let mut objects = io_string.split_terminator("\n+\n").collect::<Vec<&str>>();

    if objects.is_empty(){
        if io_string.is_empty(){
            objects = Vec::new();
        } else{
            objects = vec![io_string];
        }
    }

    Ok((from_io!(objects[0].to_string(), T)?, from_io!(objects[1].to_string(), K)?))
}
use std::fmt::Write;
#[inline]
fn handle_hashmap_iteration<T: IoDeSer, V:IoDeSer>(key:&T, value: &V, tab:u8, buffer: &mut String){
    let tabs = (0..tab + 1).map(|_| "\t").collect::<String>();

    let _ = write!(buffer, "|\n{}",&tabs);
    key.to_io_string(tab+1, buffer);
    let _ = write!(buffer, "\n{}+\n{}",&tabs, tabs);
    value.to_io_string(tab+1, buffer);
    let _ = write!(buffer, "\n{}|",(0..tab).map(|_| "\t").collect::<String>());
}
