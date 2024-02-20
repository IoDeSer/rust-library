use iodeser::*;
#[test]
fn lifetimes(){
    #[derive(IoDeSer)]
    struct Testy<'a, T: IoDeSer<'a, Output=T>>{
        pub x: T,
        pub y: &'a str
    }

    let x = Testy{ x: "reqr".to_string(), y: "www" };

    let io = to_io!(&x);
    print!("{}", &io);

}