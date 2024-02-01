mod compare;

use io_de_ser::*;

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct Test {
    pub x: String,
    pub y: i32,
}

compare_from!(&"string comparison to IO file format".to_string(), String, string);
compare_from!(&i64::MAX, i64, i64);
compare_from!(&i32::MIN, i32, i32);
compare_from!(&-5234.529348, f64, f64);
compare_from!(&vec![1,3,2224,-1232,i32::MAX], Vec<i32>, vec_i32);
compare_from!(&Test{x:"aha123".into(), y:-7}, Test, class_primitives);
compare_from!(&[-1239543,343423,0,0,0,-23445,-453,1,3,3,3,0], [i32; 12], array);
compare_from!(&[Test{x:"123test123".into(), y:0}, Test{x:"abcdwefgb".into(), y:-23854}], [Test; 2], array_class);
