

#[test]
fn test_tab() {
    use iodeser::*;
 

    let t:(i32, String, f32) = (1, "Hello".to_string(), 3.14);
    //     v: ,
    //     s: "Hello".to_string()
    // };

    println!("{:?}", t);

    let i = to_io!(t);
    println!("{}", &i);
    let x = from_io!(&i, (i32, String, f32) ).unwrap();
    assert!(x == t);
}