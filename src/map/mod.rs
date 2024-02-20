#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use crate::{delete_tabulator, from_io, IoDeSer};

macro_rules! map_to_io {
    ($s:expr, $tab: expr) => {
        {
            let mut result_io_string = String::new();
            let mut first_iteration = true;
            let tabs = (0.. $tab + 1).map(|_| "\t").collect::<String>();


            for (k, v) in $s {
                if !first_iteration{
                    result_io_string+="\n";
                    result_io_string+=&tabs.clone();
                    result_io_string+="+\n";
                }

                result_io_string+=&tabs.clone();
                    //result_io_string+="|\n";

                result_io_string+=&handle_hashmap_iteration(k,v,  $tab+1);
                    //result_io_string+=&tabs.clone();
                    //result_io_string+="|\n";


                if first_iteration{
                    first_iteration=false;
                }



            }

            format!("|\n{}\n{}|", result_io_string, (0.. $tab).map(|_| "\t").collect::<String>())
        }
    };
}
impl <'a,T:IoDeSer<'a> + Ord, K:IoDeSer<'a>> IoDeSer<'a> for BTreeMap<T, K>
where T::Output : Ord
{
    type Output = BTreeMap<T::Output, K::Output>;

    #[inline]
    fn to_io_string(&self, tab: u8) -> String {
        map_to_io!(self, tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> {
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
            .collect::<crate::Result<Self::Output>>()
    }
}

impl<'a, T: IoDeSer<'a> + Eq+ PartialEq+Hash,K: IoDeSer<'a>> IoDeSer<'a> for HashMap<T, K>
    where T::Output : Eq +Hash
{
    type Output = HashMap<T::Output, K::Output>;

    #[inline]
    fn to_io_string(&self, tab: u8) -> String {
        map_to_io!(self, tab)
    }

    fn from_io_string(io_input: &mut String) ->  crate::Result<Self::Output> {
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
            .collect::<crate::Result<Self::Output> >()
    }
}

#[inline]
fn handle_hashmap_from_io_iteration<'a,T: IoDeSer<'a>, K: IoDeSer<'a>>(io_string: &mut String)->crate::Result<(T::Output,K::Output)>{
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

#[inline]
fn handle_hashmap_iteration<'a,T: IoDeSer<'a>, V:IoDeSer<'a>>(key:&T, value: &V, tab:u8)->String{
    let tabs = (0..tab + 1).map(|_| "\t").collect::<String>();

    format!("|\n{}{}\n{}+\n{}{}\n{}|",
            /* | */
            tabs.clone(),key.to_io_string(tab+1),
            tabs.clone(), // +
            tabs.clone(), value.to_io_string(tab+1),
            (0..tab).map(|_| "\t").collect::<String>() // |
    )
}
