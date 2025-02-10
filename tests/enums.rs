#![allow(dead_code)]


#[test]
fn imports_only(){
    use iodeser::*;
    #[derive(IoDeSer, Debug)]
    enum Testowanie {
        X,
    }

}

#[test]
#[allow(dead_code)]
fn enum_test() {
    use iodeser::*;
    
    #[derive(IoDeSer, Debug)]
    #[allow(dead_code)]
    struct T1 {
        pub n: i32,
        pub f2: String,
    }

    #[derive(IoDeSer, Debug)]
    enum Testowanie<T: IoDeSer> {
        X,
        Y(i32, Vec<char>, T, T1),
        Z { b: T, a: String, c:T1 },
    }

    #[derive(IoDeSer, Debug)]
    #[allow(dead_code)]
    struct Tst<T: IoDeSer> {
        pub name: i32,
        pub choose: Testowanie<T>,
        pub choose2: Testowanie<String>,
        pub age: String,
    }


    let t = Tst{
        name: 454534,
        age: "ahaer".to_string(),
        choose: Testowanie::Z { a: "asdsad".to_string(), b: vec![-513,45,656456,4343], c: T1 { n: -34389, f2: "er4545345".to_string() } },
        choose2: Testowanie::Y(-5490, vec!['a', 'b', '0', '\'', '|'], "AeerEEEEE->df|||e".to_string(), T1 { n: 0, f2: "wwewqesdfrtht".to_string() }),
    };
    /*let t = Tst{
        name: 454534,
        age: "ahaer".to_string(),
        choose: Testowanie::Y(5445, vec!['a', 'f', '-','9','0'], 52),
    };*/
    /*let t = Tst{
        name: 454534,
        age: "ahaer".to_string(),
        choose: Testowanie::<i32>::X,
    };*/
    let f = to_io!(t);
    println!("{}\n\n======\n", &f);
    let ff = from_io!(&f, Tst<Vec<i32>>);
    println!("{:?}", ff);

}

#[cfg(feature = "serde")]
#[cfg(feature = "serde_json")]
#[cfg(feature = "chrono")]
#[test]
fn timing_Tests(){
    use serde::{Serialize, Deserialize};
    use serde_json;
    use iodeser::*;
    use std::time::Instant;
    use chrono::NaiveDate;

    #[derive(Serialize, Deserialize,IoDeSer, Debug, PartialEq)] // required macro derive IoDeSer, Debug and PartialEq is not required
    struct Person<'a> {
        #[io_name("Name")]      // optional renaming
        pub name: &'a str,
        #[io_name("SecondName")]  // optional renaming
        pub second_name: Option<&'a str>,
        #[io_name("LastName")]  // optional renaming
        pub last_name: &'a str,
        #[io_name("Age")]       // optional renaming
        #[io_order(LAST)]       // optional ordering using FIRST or LAST keyword
        pub birth_date: NaiveDate,
        #[io_name("Address")]   // optional renaming
        #[io_order(FIRST)]      // optional ordering using FIRST or LAST keyword
        pub address: Vec<Address<'a>>,
    }

    #[derive(Serialize, Deserialize,IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
    struct Address<'a> {
        #[io_order(3)]          // optional ordering using integer
        pub city: &'a str,
        #[io_order(1)]          // optional ordering using integer
        pub number: AddressNumberType<'a>,
        #[io_order(2)]          // optional ordering using integer
        pub street: &'a str,
    }

    #[derive(Serialize, Deserialize, IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
    enum AddressNumberType<'a> {
        Numeric(u16),
        String(&'a str),
    }

    let person = Person {
        name: "John",
        second_name: None,
        last_name: "Kowalski",
        birth_date: NaiveDate::from_ymd_opt(1997, 12, 29).unwrap(),
        address: vec![
            Address {
                city: "Warsaw",
                number: AddressNumberType::Numeric(65),
                street: "Tęczowa",
            },
            Address {
                city: "Hamburg",
                number: AddressNumberType::String("220a"),
                street: "Strasse",
            },
        ],
    };

    let start = Instant::now();
    let json = serde_json::to_string(&person).expect("Serialization failed");
    println!("Serialized JSON:\t{:?}", start.elapsed());

    let start = Instant::now();
    let dejson:Person = serde_json::from_str(&json).unwrap();
    println!("{:?}\n\n", start.elapsed());

    let start = Instant::now();
    let io = to_io!(&person);
    println!("Serialized io:\t{:?}", start.elapsed());



    let start = Instant::now();
    let deio:Person = from_io!(&io, Person).unwrap();
    println!("{:?}", start.elapsed());
}