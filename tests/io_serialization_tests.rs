mod compare;

use std::collections::HashMap;
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
|", i32::MAX, i32::MIN),vec_i32);

compare_to!(&Test2 { char_eg: 'M'},
"|
\tchar_eg->|M|
|", class_primitive);

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
|", class_in_class);

compare_to!(&[1,5,-231],
"|
\t|1|
\t+
\t|5|
\t+
\t|-231|
|", array);

compare_to!(&[Test2{char_eg:'Z'},Test2{char_eg:'1'}],
"|
\t|
\t\tchar_eg->|Z|
\t|
\t+
\t|
\t\tchar_eg->|1|
\t|
|", array_class);


/*compare_to!(&HashMap::from([(345435735, "true".to_string()),(-354950,"false".to_string()),(-34,"asdef".to_string())
        ,(-3123,"arrgghnghn".to_string()),(0,"skjhrkghb".to_string()),(213545,"krgjkbjtkbjt".to_string())]),
"|
\t|
\t\t|345435735|
\t\t+
\t\t|true|
\t|
\t+
\t|
\t\t|-3123|
\t\t+
\t\t|arrgghnghn|
\t|
\t+
\t|
\t\t|0|
\t\t+
\t\t|skjhrkghb|
\t|
\t+
\t|
\t\t|-34|
\t\t+
\t\t|asdef|
\t|
\t+
\t|
\t\t|213545|
\t\t+
\t\t|krgjkbjtkbjt|
\t|
\t+
\t|
\t\t|-354950|
\t\t+
\t\t|false|
\t|
|
", hashmap_primitive);*/