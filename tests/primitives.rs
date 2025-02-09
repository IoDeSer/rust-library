use iodeser::*;

#[test]
fn strings(){

    #[derive(Debug, IoDeSer, PartialEq)]
    struct Name{
        pub name: String,
        pub last_name: char,
    }

    let person = Name{name: "😀Jo||||||\nh\tn   \n".to_string(), last_name: '\n'};
    println!("{:?}\t\toriginal", person);

    let io_person = to_io!(&&&person);
    println!("'{}'\t\tio", &io_person);

    let from_io_person = from_io!(&io_person,Name).unwrap();


    println!("{:?}\t\tfrom io", from_io_person);

    assert!(person == from_io_person);

    // assert!(x.0 == z.0);
    // assert!(x.1 == z.1);
}