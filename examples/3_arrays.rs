use iodeser::*;
use std::collections::HashMap;

// This example demonstrates serialization and deserialization of iterable types such as:
//  arrays
//  vectors
//  hashmaps
// In here we also show string escaping from special characters such as new line (\n),
//  vertical bars (|), tabulator (\t) and so on.
// This example will also touch on generic types <T>.

fn main() {
    // initialization of objects
    let array: [i32; 4] = [5,7,3,-55]; // array
    let vector: Vec<i32> = vec![-100, i32::MAX, 0, 44]; // vector with generic type i32
    let mut hashmap: HashMap<&str, char> = HashMap::new(); // hashmap with generic type &str and char

    hashmap.insert("empty_key", ' ');
    hashmap.insert("newline_key", '\n'); 
    hashmap.insert("vertical_bar_key", '|'); // notice special characters \n and |

    // serialization
    let array_serialized = to_io!(&array);
    let vector_serialized = to_io!(&vector);
    let hashmap_serialized = to_io!(&hashmap);

    // printing to console serialized strings. Notice how special characters were serialized in hashmap type.
    println!("Array:\n{}\n\nVector:\n{}\n\nHashmap:\n{}", array_serialized, vector_serialized, hashmap_serialized);

    // deserialization:
    //  array:
    let array_deserialized = from_io!(array_serialized, [i32; 4]).unwrap();
    // the second parameter must hold valid array type with its elements type and correct lenght.
    // Note: Uncomment this line to see what happened if wrong lenght is passed:
    // let array_deserialized_wrong = from_io!(array_serialized, [i32; 5]).unwrap();
    assert_eq!(array, array_deserialized);

    //  vector:
    let vector_deserialized = from_io!(vector_serialized, Vec<i32>).unwrap();
    // When deserialized object has generic types in its definition (like Vec<T>, T being generic type)
    //  we also need to specify that generic type in macro 'from_io'.

    // Note: Generic type does not need to exactly match the original definition. For example,
    //  when serializing Vec<i8>, we can deserialize it using Vec<i16> or Vec<i32> and so on,
    //  as the elements type of the vectors are "compatable". Uncomment the below lines to check:
    //  let vector_deserialized_also_correct = from_io!(vector_serialized, Vec<i64>).unwrap();
    //  let vector_deserialized_wrong = from_io!(vector_serialized, Vec<i16>).unwrap(); //  does not work, because there is an element in 'vector_serialized' bigger than i16::MAX, otherwise it would work.
    assert_eq!(vector, vector_deserialized);

    //  hashmap:
    let hashmap_deserialized = from_io!(hashmap_serialized, HashMap<&str, char>).unwrap();
    assert_eq!(hashmap, hashmap_deserialized);
}
/* Output:
Array:
|
        |5|
        +
        |7|
        +
        |3|
        +
        |-55|
|

Vector:
|
        |-100|
        +
        |2147483647|
        +
        |0|
        +
        |44|
|

Hashmap:
|
        |
                |key1|
                +
                | |
        |
        +
        |
                |key3|
                +
                |\n|
        |
        +
        |
                |key4|
                +
                ||||
        |
|
*/