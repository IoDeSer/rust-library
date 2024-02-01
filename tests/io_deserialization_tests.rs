use io_de_ser::*;

#[test]
fn string_deserialization(){
    let s = "this is a string".to_string();
    let io_out = to_io!(s.clone());

    let s_new = from_io!(io_out, String);
    assert_eq!(s_new, s);
}



#[test]
fn numbers_deserialization(){
    let i = i64::MAX;
    let io_out = to_io!(i);

    let i_new = from_io!(io_out, i64);
    assert_eq!(i_new, i);
}


#[test]
fn numbers2_deserialization(){
    let i = i32::MIN;
    let io_out = to_io!(i);

    let i_new = from_io!(io_out, i32);
    assert_eq!(i_new, i);
}

#[test]
fn float_deserialization(){
    let f = -5234.529348;
    let io_out = to_io!(f);

    let f_new = from_io!(io_out, f64);
    assert_eq!(f_new, f);
}

#[test]
fn vector_deserialization(){
    let v = vec![1,3,2224,-1232,i32::MAX];
    let io_out = to_io!(v.clone());

    let v_new = from_io!(io_out, Vec<i32>);
    assert_eq!(v_new, v);
}

#[test]
fn class_deserialization(){
    #[derive(IoDeSer, Debug, PartialEq, Clone)]
    struct Test{
        pub x: String,
        pub y: i32,
    }

    let t = Test{x:"aha123".into(), y:-7};
    let io_out = to_io!(t.clone());

    let t_new = from_io!(io_out, Test);
    assert_eq!(t_new, t);
}