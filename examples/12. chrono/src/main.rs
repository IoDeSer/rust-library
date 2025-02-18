use iodeser::*;
use chrono::{NaiveDate, Utc, DateTime};


fn main() {
    let naive_date = NaiveDate::from_ymd_opt(2020, 3, 12).unwrap();
    let io_string = to_io!(&naive_date);

    assert_eq!(naive_date, from_io!(io_string, NaiveDate).unwrap());


    let now = Utc::now();
    let io_string = to_io!(&now);

    assert_eq!(now, from_io!(io_string, DateTime<Utc>).unwrap());
}