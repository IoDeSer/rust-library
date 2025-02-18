// This example demonstrates importing, serialization and deserialization of types
//  from chrono crate (https://docs.rs/chrono/latest/chrono).


/* In order to use chrono feature from IoDeSer crate, set feature flag in Cargo.toml:
    [dependencies]
    io_deser = {version = "*", features=["chrono"] }
*/

use iodeser::*;
use iodeser::chrono::{NaiveDate, Utc, DateTime};

fn main() {
    let naive_date = NaiveDate::from_ymd_opt(2020, 3, 12).unwrap();
    let io_string = to_io!(&naive_date);
    
    assert_eq!(naive_date, from_io!(io_string, NaiveDate).unwrap());

    let now = Utc::now();
    let io_string = to_io!(&now);

    assert_eq!(now, from_io!(io_string, DateTime<Utc>).unwrap());
}