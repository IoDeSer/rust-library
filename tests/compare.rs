use std::fmt::Debug;

#[macro_export]
macro_rules! compare_from {
    ($original: expr,$type:ty, $name:expr) => {
        ::paste::paste!{
            #[test]
            fn [<deserialization_ $name>]()
            {
                let io_out = to_io!($original);
                let copy = from_io!(io_out, $type);
                println!("DESERIALIZATION TEST FOR {}", stringify!($name));
                println!("{:?}", $original);
                println!("vs");
                println!("{:?}", &copy);
                println!();
                assert_eq!($original, copy);
            }
        }
    };
}

#[macro_export]
macro_rules! compare_to{
    ($original: expr, $proper_outcome:expr, $name:expr) => {
        ::paste::paste!{
            #[test]
            fn [<serialization_ $name>]()
            {
                println!("SERIALIZATION TEST FOR {}", stringify!($name));
                let out = to_io!($original);
                println!("{}", &out);
                println!("====\n vs\n====");
                println!("{}",$proper_outcome);
                println!();
                assert_eq!(out, $proper_outcome);
            }
        }
    };
}