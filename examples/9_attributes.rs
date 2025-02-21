use iodeser::*;

// This example demonstrates the use of struct attibutes:
//  #[io_name],
//  #[io_order],
//  #[io_ignore],
//  #[io_allow],


// Those attributes are used on fields in struct.
// io_name      -   Renaming fields for serialization and deserialization. Takes String parameter.
// io_order     -   Changing order of fields in serialization. Takes u16 parameter.
//                      If this attribute is being used, all fields must have it.
// io_ignore    -   Ignore public field in de/serialization.
//                      The type need to implement trait Default.
// io_allow     -   Allow private field to be de/serialized.

// They can be used, when deserializing from a file that has 
//  either different order,
//  when field names are different or that would cause errors/warnings in rust,

// For serializing, when next API using serialize data needs different names or order


#[derive(IoDeSer)]
struct HttpRequest<T: IoDeSer>{
    #[io_allow] route_id:u32,
    #[io_ignore] pub home_page: String,
    #[io_name("r")] #[io_order(1)] pub route: String,
    #[io_name("d")] #[io_order(0)] pub data: T
}

#[derive(IoDeSer)]
struct HttpRequestUnchanged<T: IoDeSer>{
    #[io_allow] route_id:u32,
    #[io_ignore] pub home_page: String,
    pub route: String,
    pub data: T
}

fn main() {
    // Sample serialized data from an API with field names "d" and "r"
    let http_request_serialized = 
"|
    route_id->|5|
    d->512
    r->\"www.data.com\"
|".to_string();

    // Instead of changing names in our struct, we can use attribute '#[io_name(...)]'
    //  to match these fields (r=route and d=data).
    //  After this, we can safely deserialize:
    let _http_request_deserialized = from_io!(http_request_serialized, HttpRequest<String>).unwrap();



    // Similarly, if API needs certain field names, that are differnt from rust definition:
    let http_request = HttpRequest::<char>{ route_id:5, home_page:"/".into(), route:"/set/book".into(), data:'Z' };
    let serialized = to_io!(&http_request);
    assert_eq!(serialized, "|\n\troute_id->|5|\n\td->|Z|\n\tr->|/set/book|\n|");
    println!("{}\n", serialized);

    // Compare above output, to the one below:
    let http_request_unchanged = HttpRequestUnchanged::<char>{ route_id:5, home_page:"/".into(), route:"/set/book".into(), data:'Z' };
    let serialized_unchanged = to_io!(&http_request_unchanged);
    println!("{}", serialized_unchanged);
}
/*Output:
|
        d->|Z|
        r->|/set/book|
|

|
        route->|/set/book|
        data->|Z|
|
*/