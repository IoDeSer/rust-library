mod compare;

use std::collections::{BTreeMap, HashMap, HashSet, LinkedList, VecDeque};
use iodeser::*;

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct Parent<T: IoDeSer>{
    #[io_order(LAST)]
    #[io_name("TestClass")]
    pub x:Test,
    pub y: T,
    #[io_order(FIRST)]
    pub z: String
}

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct EmptyCheck{
    pub x:HashMap<i32, char>,
    pub y: Vec<String>,
    pub z: Vec<u8>,
}

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct Test {
    pub x: String,
    pub y: i32,
}

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct Test2<T :IoDeSer>{
    pub x: T,
    pub y: String,
}

compare_from!(&"string comparison to IO file format".to_string(), String, string);
compare_from!(&i64::MAX, i64, i64);
compare_from!(&i32::MIN, i32, i32);
compare_from!(&-5234.529348, f64, f64);
compare_from!(&vec![1,3,2224,-1232,i32::MAX], Vec<i32>, vec_i32);
compare_from!(&Test{x:"aha123".into(), y:-7}, Test, class_primitives);
compare_from!(&[-1239543,343423,0,0,0,-23445,-453,1,3,3,3,0], [i32; 12], array);
compare_from!(&[Test{x:"123test123".into(), y:0}, Test{x:"abcdwefgb".into(), y:-23854}], [Test; 2], array_class);

compare_from!(&HashMap::from([
    (123543545,"This is a text sequence in hashmap".to_string())]), HashMap<i32, String>, hashmap_primitive);
compare_from!(&HashMap::from([
    (123543545,Test{x: "asdffghth3444".to_string(),y: 1233}),
    (-2,       Test{x: "dfdre".to_string(),y: 23})]), HashMap<i32, Test>, hashmap_class);

compare_from!(&Test2{x: true, y: "testing".to_string()}, Test2<bool>, class_generic);
compare_from!(&Parent{x:Test{x:"inside".to_string(), y:5},y:-1,z:"string test".to_string()}, Parent<i32>, class_in_class);
compare_from!(&Parent::<Vec<bool>>{x:Test{x:"inside".to_string(), y:5},y:vec![true, false, false,false,true,true],z:"string test".to_string()}, Parent<Vec<bool>>, class_in_class_generic);
compare_from!(&Vec::<char>::new(), Vec<char>, vec_empty);
compare_from!(&Parent::<Vec<String>>{x:Test{x:"inside".to_string(), y:5},y:vec![],z:"string test".to_string()}, Parent<Vec<String>>, class_in_class_generic_empty_vec);
compare_from!(&[0u8; 0], [u8; 0], array_empty);
compare_from!(&HashMap::<String, u8>::new(), HashMap<String, u8>, hashmap_empty);
compare_from!(&EmptyCheck{x: HashMap::new(), y:Vec::new(), z:Vec::new()}, EmptyCheck, class_full_empty);
compare_from!(&BTreeMap::from([
    ("Mercury".to_string(), 0.4),
    ("Venus".to_string(), 0.7),
    ("Earth".to_string(), 1.0),
    ("Mars".to_string(), 1.5),
]), BTreeMap<String, f32>, b_tree_map);
compare_from!(&LinkedList::from([1,5,4,3,76]), LinkedList<i32>, linked_list);
compare_from!(&VecDeque::from([1,5,4,3,76]), VecDeque<i32>, vec_queue);
compare_from!(&HashSet::from([1,5,4,3,76]), HashSet<i32>, hash_set);
