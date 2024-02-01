mod compare;

use io_de_ser::*;

#[derive(IoDeSer, Debug, Default)]
struct Person {
    pub name: String,
    pub age: i32,
    pub test: Test,
}

/*#[derive(IoDeSer, Debug)]
struct Val<T>{
    pub v: T,
    pub s: String
}
*/
#[derive(IoDeSer, Debug, Default)]
struct Test {
    pub year: u64,
    pub test2: Test2,
}

#[derive(IoDeSer, Debug, Default)]
struct Test2 {
    pub char_eg: char,
}


compare_to!(&vec![1,5,-1232,i32::MAX, i32::MIN, 0,0,0,0,0,-53539,123],
format!("|
\t|1|
\t+
\t|5|
\t+
\t|-1232|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|0|
\t+
\t|0|
\t+
\t|0|
\t+
\t|0|
\t+
\t|0|
\t+
\t|-53539|
\t+
\t|123|
|", i32::MAX, i32::MIN),Veci32);

compare_to!(&Test2 { char_eg: 'M'},
"|
\tchar_eg->|M|
|", ClassPrimitive);

compare_to!(&Person { name: "example_name".to_string(), age: 1, test: Test { year: 2023, test2: Test2 { char_eg: 'z' } } },
"|
\tname->|example_name|
\tage->|1|
\ttest->|
\t\tyear->|2023|
\t\ttest2->|
\t\t\tchar_eg->|z|
\t\t|
\t|
|", ClassInClass);
