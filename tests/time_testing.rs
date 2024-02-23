use std::time::{Duration, SystemTime, UNIX_EPOCH};
use iodeser::*;
#[test]
fn system_time(){
    let now = SystemTime::now();
    let io = to_io!(now);

    println!("{}", &io);

    let prv =from_io!(io, SystemTime).unwrap();

    println!("{:?}", now.duration_since(UNIX_EPOCH).unwrap().as_nanos());
    println!("{:?}", prv.duration_since(UNIX_EPOCH).unwrap().as_nanos());
}
#[cfg(feature = "chrono")]
#[test]
fn naive_time(){
    assert!(cfg!(feature = "chrono"));
    use chrono::{NaiveTime};

    let io = "|1970-01-01T09:25:00.774658300+00:00|".to_string();
    let from = from_io!(io, NaiveTime);
    println!("{:?}", &from);
}
#[test]
fn duration(){
    let x = Duration::from_secs(12378);
    println!("{}",to_io!(&x));
}