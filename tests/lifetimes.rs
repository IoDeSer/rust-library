use iodeser::*;
#[test]
fn lifetimes(){
    #[derive(IoDeSer, Debug)]
    struct Testy<'a, T: IoDeSer>{
        pub x: &'a mut T,
        pub y: &'a str
    }

    #[derive(IoDeSer, Debug)]
    struct Testy2<'a>{
        pub x: &'a mut Testy<'a, String>,
        pub y: char
    }

    let mut x2 = Testy{ x: & mut "reqr".to_string(), y: "www" };
    let x = Testy2{ x:&mut x2, y: 'a' };

    let io = to_io!(&x);
    println!("{}", &io);

    let x2 = from_io!(io, Testy2);
    println!("{:?}", &x2);
}