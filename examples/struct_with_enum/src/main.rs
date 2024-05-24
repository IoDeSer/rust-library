use iodeser::*; // required import
use chrono::NaiveDate;

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive IoDeSer, Debug and PartialEq is not required
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

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
struct Address<'a> {
    #[io_order(3)]          // optional ordering using integer
    pub city: &'a str,
    #[io_order(1)]          // optional ordering using integer
    pub number: AddressNumberType<'a>,
    #[io_order(2)]          // optional ordering using integer
    pub street: &'a str,
}

#[derive(IoDeSer, Debug, PartialEq)] // required macro derive, Debug and PartialEq is not required
enum AddressNumberType<'a>{
    Numeric(u16),
    String(&'a str)
}

fn main() {
    let person = Person {
        name: "John",
        second_name: None,
        last_name: "Kowalski",
        birth_date: NaiveDate::from_ymd_opt(1997, 12,29).unwrap(),
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

    let io_serialization: String = to_io!(&person); // serialization
    let person_deserialization: Person = from_io!(io_serialization, Person).unwrap(); // deserialization

    assert_eq!(person, person_deserialization);
}