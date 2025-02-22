use iodeser::*;

#[test]
#[allow(dead_code)]
//#[should_panic]
pub fn struct_with_attributes(){
    #[derive(IoDeSer, Debug)]
    struct Person{
        #[io_allow] #[io_name("PersonID")]
        id: u16,

        //#[io_ignore] #[io_order(5)]
        id2: String,

        //#[io_allow] // this would causing error 
        pub name: String,

        //#[io_name("5")] #[io_ignore] 
        pub last_name: String
    }
}

#[test]
#[allow(dead_code)]
fn unit_struct(){
    #[derive(IoDeSer, Debug, Default)]
    struct X;

    #[derive(IoDeSer, Debug)]
    struct Person{
        #[io_allow] #[io_name("PersonID")]     
        id: u16,

        #[io_allow]
        id2: Option<String>,

        pub name: String,
        pub last_name: String,
        pub check:X
    }

    let x = Person{id:66, id2: None, name:"this is a name!".into(),last_name:"last name....".into(), check: X};

    println!("{x:?}");

    let io = to_io!(&x);

    println!("\n{io}\n========\n");

    let x2 = from_io!(io, Person).unwrap();

    println!("{x2:?}");
}