mod compare;

use io_de_ser::*;

#[derive(IoDeSer, Debug, PartialEq, Clone)]
struct Test {
    pub x: String,
    pub y: i32,
}

compare_from!("string comparison to IO file format".to_string(), String, String);
compare_from!(i64::MAX, i64, i64);
compare_from!(i32::MIN, i32, i32);
compare_from!(-5234.529348, f64, f64);
compare_from!(vec![1,3,2224,-1232,i32::MAX], Vec<i32>, Veci32);
compare_from!(Test{x:"aha123".into(), y:-7}, Test, ClassPrimitives);
