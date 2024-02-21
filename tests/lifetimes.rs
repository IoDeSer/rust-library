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

#[test]
fn slices(){
    #[derive(IoDeSer, Debug, PartialEq)]
    struct Testy<'a>{
        pub x: i32,
        pub y: &'a str
    }

    let mut x = [Testy{ x: 6545, y: "sdr4t" }, Testy{ x: -54, y: "eterg" }, Testy{ x: 34, y: "EEEq2" }];
    let x1= x.as_mut_slice();
    let io = to_io!(x1);
    println!("{}", &io);

    let x2 = from_io!(io, &[Testy]).unwrap();
    println!("{:?}", &x1);
    println!("{:?}", &x2);
    assert_eq!(x1, x2);
}