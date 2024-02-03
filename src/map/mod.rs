use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use crate::{delete_tabulator, from_io, IoDeSer};

impl <T:IoDeSer, K:IoDeSer> IoDeSer for BTreeMap<T, K>{
    fn to_io_string(&self, tab: u8) -> String {
        todo!()
    }

    fn from_io_string(io_input: &mut String) -> Self {
        todo!()
    }
}

impl<T: IoDeSer + Eq+ PartialEq+Hash,K: IoDeSer> IoDeSer for HashMap<T, K>{
    //type Type = HashMap<T, K>;
    fn to_io_string(&self, tab: u8) -> String {
        let mut result_io_string = String::new();
        let mut first_iteration = true;
        let tabs = (0..tab + 1).map(|_| "\t").collect::<String>();

        /* TODO

        for empty create:
        |

        |

        and not:
        |
        |

         */


        for (k, v) in self {
            if !first_iteration{
                result_io_string+="\n";
                result_io_string+=&tabs.clone();
                result_io_string+="+\n";
            }

            result_io_string+=&tabs.clone();
            //result_io_string+="|\n";

            result_io_string+=&handle_hashmap_iteration(k,v, tab+1);
            //result_io_string+=&tabs.clone();
            //result_io_string+="|\n";


            if first_iteration{
                first_iteration=false;
            }



        }

        format!("|\n{}\n{}|", result_io_string, (0..tab).map(|_| "\t").collect::<String>())
    }

    fn from_io_string(io_input: &mut String) -> HashMap<T, K> {
        delete_tabulator(io_input);
        let mut objects = io_input.split_terminator("\n+\n").collect::<Vec<&str>>();
        if objects.is_empty(){
            if io_input.is_empty(){
                objects = Vec::new();
            } else{
                objects = vec![io_input];
            }
        }

        let mut map = HashMap::<T, K>::new();

        for o in objects.iter_mut() {
            let x: (T,K) = handle_hashmap_from_io_iteration( &mut o.to_string());
            map.insert(x.0, x.1);
        }

        map
    }
}

fn handle_hashmap_from_io_iteration<T: IoDeSer, K: IoDeSer>(io_string: &mut String)->(T,K){
    delete_tabulator(io_string);
    let mut objects = io_string.split_terminator("\n+\n").collect::<Vec<&str>>();

    if objects.is_empty(){
        if io_string.is_empty(){
            objects = Vec::new();
        } else{
            objects = vec![io_string];
        }
    }

    (from_io!(objects[0].to_string(), T), from_io!(objects[1].to_string(), K))
}

fn handle_hashmap_iteration<T: IoDeSer, V:IoDeSer>(key:&T, value: &V, tab:u8)->String{
    let tabs = (0..tab + 1).map(|_| "\t").collect::<String>();

    format!("|\n{}{}\n{}+\n{}{}\n{}|",
            /* | */
            tabs.clone(),key.to_io_string(tab+1),
            tabs.clone(), // +
            tabs.clone(), value.to_io_string(tab+1),
            (0..tab).map(|_| "\t").collect::<String>() // |
    )
}