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

#[test]
fn serialization_Veci32(){
    let v = vec![1,5,-1232,i32::MAX, i32::MIN, 0,0,0,0,0,-53539,123];

    assert_eq!(to_io!(v.clone()), format!(
"|
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
\t+
\t|{}|
|", v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7],v[8],v[9],v[10],v[11]));
}


compare_to!(Test2 { char_eg: 'M'},
"|
\tchar_eg->|M|
|", ClassPrimitive);

compare_to!(Person { name: "example_name".to_string(), age: 1, test: Test { year: 2023, test2: Test2 { char_eg: 'z' } } },
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
