#![allow(dead_code)]

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