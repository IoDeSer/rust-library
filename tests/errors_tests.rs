use iodeser::*;

#[derive(IoDeSer, Debug)]
struct Test{
    pub x:i32
}

#[test]
fn stru(){
    let io = to_io!(&5);
    let x1 = from_io!(io, Test);



    println!("{:?}", &x1);
}

#[test]
fn array(){
    let f:[i32;0] = [];
    let io = to_io!(&f);
    println!("{:?}", &io);

    let x1 = from_io!(io, [i32; 2]);
    println!("{:?}", &x1);
}

#[test]
fn vec(){
    let io = to_io!(&vec![0,43,-3,23,0,0]);
    println!("{:?}", &io);

    let x1 = from_io!(io, Vec<i32>);
    println!("{:?}", &x1);
}