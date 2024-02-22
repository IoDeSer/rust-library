use std::time::{SystemTime, UNIX_EPOCH};
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