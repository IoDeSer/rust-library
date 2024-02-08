use iodeser::*;


#[test]
fn order_with_tuple_struct(){
    #[derive(IoDeSer, Debug)]
    struct Test(
        #[io_order(LAST)]
        pub i32,
        pub i32
    );
    println!("{}", to_io!(&Test(5,-320)));
}

#[test]
fn primitives(){
    #[derive(IoDeSer, Debug)]
    struct Create(pub String, pub i32, pub f32, pub char);


    let x = Create("string inside".to_string(), 505, -0.52, 'a');
    let io = to_io!(&x);
    println!("{}", &io);
    assert_eq!(io,
"|
\t|string inside|
\t+
\t|505|
\t+
\t|-0.52|
\t+
\t|a|
|");
}

#[test]
fn struct_in_struct_with_ordering(){
    #[derive(IoDeSer, Debug)]
    struct Test{
        #[io_order(LAST)]
        pub z:char,
        pub y:u8,
        #[io_order(FIRST)]
        pub x:i32,
    }
    #[derive(IoDeSer, Debug)]
    struct Create2(pub String, pub i32, pub Test, pub f32, pub char);


    let x = Create2("string inside version 2".to_string(), 505,Test{x:5, y:1, z:'y'}, -0.52, 'a');
    let io = to_io!(&x);
    println!("{}", &io);
    println!("{:?}", &x);
    assert_eq!(io,
"|
\t|string inside version 2|
\t+
\t|505|
\t+
\t|
\t\tx->|5|
\t\ty->|1|
\t\tz->|y|
\t|
\t+
\t|-0.52|
\t+
\t|a|
|");
}

#[test]
fn struct_in_struct_generic(){
    #[derive(IoDeSer, Debug)]
    struct Test<T:IoDeSer, Y:IoDeSer>{
        pub x:T,
        pub y:Y,
        pub z:char
    }
    #[derive(IoDeSer, Debug)]
    struct Create2<T: IoDeSer, K:IoDeSer>(pub String, pub i32, pub Test<K,u8>, pub T, pub char);


    let x = Create2("string inside version 3".to_string(), 505,Test{x:5, y:1, z:'y'}, -0.52, 'a');
    let io = to_io!(&x);
    println!("{}", &io);
    assert_eq!(io,
               "|
\t|string inside version 3|
\t+
\t|505|
\t+
\t|
\t\tx->|5|
\t\ty->|1|
\t\tz->|y|
\t|
\t+
\t|-0.52|
\t+
\t|a|
|");
}

#[test]
fn struct_tuple_in_struct(){
    #[derive(IoDeSer, Debug)]
    struct Test<T:IoDeSer>(pub i32,pub T,pub String);
    #[derive(IoDeSer, Debug)]
    struct Create2<T: IoDeSer, K:IoDeSer>(pub String, pub i32, pub Test<K>, pub T, pub char);


    let x = Create2("string inside version 4".to_string(), 505,Test(4545354, <f32>::MIN, "TESTING".to_string()), -100000.324, 'H');
    let io = to_io!(&x);
    println!("{}", &io);
    assert_eq!(io,
               "|
\t|string inside version 4|
\t+
\t|505|
\t+
\t|
\t\t|4545354|
\t\t+
\t\t|-340282350000000000000000000000000000000|
\t\t+
\t\t|TESTING|
\t|
\t+
\t|-100000.324|
\t+
\t|H|
|");
}

#[test]
fn struct_tuple_in_struct_deserialization(){
    #[derive(IoDeSer, Debug, PartialEq)]
    struct Test<T:IoDeSer>(pub i32,pub T,pub String);
    #[derive(IoDeSer, Debug, PartialEq)]
    struct Create2<T: IoDeSer, K:IoDeSer>(pub String, pub i32, pub Test<K>, pub T, pub char);


    let x = Create2("string inside version 5".to_string(), 505,Test(4545354, <f32>::MIN, "TESTING".to_string()), -100000.324, 'H');
    let io = to_io!(&x);

    let x2 = from_io!(io, Create2<f32, f32>).unwrap();
    println!("{:?}", x);
    println!("{:?}", x2);
    assert_eq!(x,x2);
}