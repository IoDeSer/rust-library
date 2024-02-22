mod compare;

use std::collections::{HashMap, LinkedList, VecDeque};
use iodeser::*;

#[derive(IoDeSer, Debug, Default)]
struct Person {
    pub name: String,
    pub age: i32,
    pub test: Test,
}

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

compare_to!(&Test2 { char_eg: '❤'},
"|
\tchar_eg->|❤|
|", class_primitive);

compare_to!(&Person { name: "example_name".to_string(), age: 1, test: Test { year: 2023, test2: Test2 { char_eg: '😍' } } },
"|
\tname->|example_name|
\tage->|1|
\ttest->|
\t\tyear->|2023|
\t\ttest2->|
\t\t\tchar_eg->|😍|
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

compare_to!(&(1, true, "testing?".to_string()),
"|
\t|1|
\t+
\t|true|
\t+
\t|testing?|
|", tuple_primitives);

compare_to!(&VecDeque::from([1,5,-231]),
"|
\t|1|
\t+
\t|5|
\t+
\t|-231|
|", vec_queue);

compare_to!(&LinkedList::from([1,5,-231]),
"|
\t|1|
\t+
\t|5|
\t+
\t|-231|
|", linked_list);

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


compare_to!(&HashMap::from([(345435735, "true".to_string())]),
"|
\t|
\t\t|345435735|
\t\t+
\t\t|true|
\t|
|", hashmap_primitive);

compare_to!(&HashMap::from([(345435735, Test2{char_eg:'i'})]),
"|
\t|
\t\t|345435735|
\t\t+
\t\t|
\t\t\tchar_eg->|i|
\t\t|
\t|
|", hashmap_class);

compare_to!(&[0i32;0],
"|

|", array_empty);

compare_to!(&Vec::<char>::new(),
"|

|", vec_empty);

compare_to!(&HashMap::<char, i32>::new(),
"|

|", hashmap_empty);