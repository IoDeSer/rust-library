use std::collections::HashMap;


#[test]
fn test_maps(){
    use iodeser::*;

    let t    = HashMap::<char, i32>::new();
    println!("{:?}\t ORG", t);

    let i = to_io!(t);
    println!("{}", &i);
    let x = from_io!(&i, HashMap<char, i32> ).unwrap();
    println!("{:?}\t OUT", &x);
    assert!(x == t);
}

#[test]
fn test_tab() {
    use iodeser::*;

    #[derive(IoDeSer, Debug, PartialEq)]
    struct Person{
        pub addresses1: Option<Vec<String>>,
        pub addresses2: Option<Vec<String>>,
        pub addresses3: Vec<String>,
        pub addresses4: Option<Vec<String>>,
        pub addresses5: Option<Vec<String>>,
        pub addresses6: Vec<String>,

        pub name:String
    }
 
    let t = Person{
        addresses1: Some(vec![]),
        addresses2: None,
        addresses3: vec![],
        addresses4: Some(vec!["a".to_string(), "b".to_string()]),
        addresses5: Some(vec!["asd".to_string()]),
        addresses6: vec!["Hello!".to_string(), "World!".to_string()],

        name: "John".to_string()
    };

    println!("{:?}\t ORG", t);

    let i = to_io!(t);
    println!("{}", &i);
    let x = from_io!(&i, Person ).unwrap();
    println!("{:?}\t OUT", &x);
    assert!(x == t);
}