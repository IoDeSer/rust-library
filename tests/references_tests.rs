use std::sync::{Arc, Mutex};
use iodeser::*;

#[test]
fn arc(){
    #[derive(Debug, IoDeSer, PartialEq)]
    struct TestArc{
        pub x:String,
        pub y:i32
    }
    let a = Arc::new(TestArc{ x: "test".to_string(), y: 34421 });

    let io = to_io!(&a);
    println!("{}", &io);

    let a2 = from_io!(io, Arc<TestArc>).unwrap();
    println!("{:?}", a.as_ref());
    println!("{:?}", a2.as_ref());

    assert_eq!(format!("{:?}", a.as_ref()),format!("{:?}", a2.as_ref()))
}

#[test]
fn arc_mutex(){
    #[derive(Debug, IoDeSer, PartialEq)]
    struct TestArc{
        pub x:String,
        pub y:i32
    }
    let a = Arc::new(Mutex::new(TestArc{ x: "test".to_string(), y: -3123 }));

    let io = to_io!(&a);
    println!("{}", &io);

    let a2 = from_io!(io, Arc<Mutex<TestArc>>).unwrap();

    {
        println!("{:?}", a.as_ref().lock().unwrap());
        println!("{:?}", a2.as_ref().lock().unwrap());
    }

    assert_eq!(format!("{:?}", a.as_ref().lock().unwrap()),format!("{:?}", a2.as_ref().lock().unwrap()))
}

#[test]
fn box_test(){
    #[derive(Debug, IoDeSer, PartialEq)]
    struct TestBox{
        pub x:String,
        pub y:u8,
        pub z:char
    }

    let a = Box::new(TestBox{ x: "test".to_string(), y: 1, z:'a' });

    let io = to_io!(&a);
    println!("{}", &io);

    let a2 = from_io!(io, Box<TestBox>).unwrap();

    println!("{:?}", &a);
    println!("{:?}", &a2);


    assert_eq!(format!("{:?}", a),format!("{:?}", a2))
}
