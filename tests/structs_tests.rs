use iodeser::*;

#[test]
#[allow(dead_code)]
pub fn struct_with_attributes(){
    #[derive(IoDeSer, Debug)]
    struct Person{
        #[io_allow] #[io_name("PersonID")]
        id: u16,
        id2: String,

        // #[io_allow] // this would causing error
        pub name: String,
        #[io_ignore]
        pub last_name: String
    }

    let p = Person{id:55, id2: "--2321--sdf".into(), name:"name".into(), last_name: "last".into()};
    let io = to_io!(&p);

    println!("{}", io);

    let d = from_io!(io, Person).unwrap();

    println!("\n\n{:?}\n{:?}", p, d);
}