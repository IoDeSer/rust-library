use iodeser::*;

#[derive(IoDeSer, Debug)]
struct Test{
    pub x:i32
}

#[derive(IoDeSer)]
struct Create(pub String);



#[test]
fn class(){
    let io = to_io!(&5);
    let x1 = from_io!(io, Test);

    println!("{:?}", &x1);
}

#[test]
fn array(){
    let f:[i32;0] = [];
    let io = to_io!(&f);

    match from_io!(io, [i32; 2]){
        Ok(o) => println!("{:?}", &o),
        Err(e) => println!("{}", &e),
    }
}

#[test]
fn vec(){
    let io = to_io!(&5);

    let x1 = from_io!(io, bool);
    println!("vec:\t{:?}", &x1);
}

#[test]
fn primitive(){
    let io = to_io!(&"test".to_string());

    let x1 = from_io!(io, i32);
    println!("prim:\t{:?}", &x1);
}

#[test]
fn tuple(){
    let t = (1,true);
    let io = to_io!(&t);

    println!("{}", &io);
    let tt = from_io!(io, (i32,u8));
    println!("{:?}", tt);
}

#[test]
fn checks(){
    let io = to_io!(&"test".to_string());

    match from_io!(io, i32){
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}")
    }
}