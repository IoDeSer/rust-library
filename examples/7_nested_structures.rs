use iodeser::*;

// This example demonstrates serialization and deserialization of nested structures
//  like an enum as a filed of a struct.

// We want to create and serialize 'AnimalShelter' object. 'AnimalShelter' struct
//  has an address (struc Address) and a vector with animals (enum Animal).

// NOTE: All fields in a struct/enum must implement IoDeSer trait and if that field is private
//  it must also implement Default trait.


#[derive(IoDeSer, Debug, PartialEq)] // Debug and PartialEq traits are only required for assert_eq!
struct AnimalShelter{
    address: Address,       // private field must implement Default trait
    pub animals: Vec<Animal>
}

#[derive(IoDeSer, Default, Debug, PartialEq)] // must implement Default trait
struct Address{
    pub city: String,   // on deserialization, will default to ""
    pub street: String  // on deserialization, will default to ""
}

#[derive(IoDeSer, Debug, PartialEq)] 
enum Animal{
    Cat, 
    Dog{breed: String}, 
    Bunny(i32),
}

fn main() {
    // initialize object
    let animal_shelter = AnimalShelter{
        address:
            Address{ city:"Helsinki".into(), street:"Bulevardi".into() },
        animals: vec![
            Animal::Cat,
            Animal::Cat,
            Animal::Dog{breed: "Chihuahua".into()},
            Animal::Cat,
            Animal::Bunny(8),
            Animal::Dog{breed: "Shiba Inu".into()}
        ]};

    // serialize
    let animal_shelter_serialize = to_io!(&animal_shelter);
    println!("{}", animal_shelter_serialize); // Notice how output does not show 'address' field, as its private and not serialized

    // deserialize
    let animal_shelter_deserialize = from_io!(animal_shelter_serialize, AnimalShelter).unwrap();
    println!("{:?}", animal_shelter_deserialize);

    assert_eq!(animal_shelter.animals, animal_shelter_deserialize.animals);
}
/*Output:
|
        animals->|
                |
                        Cat->|||
                |
                +
                |
                        Cat->|||
                |
                +
                |
                        Dog->|
                                breed->|Chihuahua|
                        |
                |
                +
                |
                        Cat->|||
                |
                +
                |
                        Bunny->|
                                |8|
                        |
                |
                +
                |
                        Dog->|
                                breed->|Shiba Inu|
                        |
                |
        |
|
AnimalShelter { address: Address { city: "", street: "" }, animals: [Cat, Cat, Dog { breed: "Chihuahua" }, Cat, Bunny(8), Dog { breed: "Shiba Inu" }] }
*/