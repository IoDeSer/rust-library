#[cfg(feature = "serde")]
mod serde_tests {
    use iodeser::serde::to_string; // Your Serde-compatible function
    use serde::Serialize;

    #[test]
    fn test_serialize_primitives() {
        

        let p = 
            42;
            // true;
            //"hello".to_string()
        

        let output = to_string(&p).expect("Failed to serialize");
        println!("{output}")
    }
}