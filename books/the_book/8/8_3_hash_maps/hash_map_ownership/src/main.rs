use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    println!("{field_name}: {field_value}");
    // Favourite colour: Blue

    // Create a new empty hash map
    let mut map = HashMap::new();

    // String types are moved into map, not copied like i32 etc.
    map.insert(field_name, field_value);
    // field_name and field_value variables not available now

    // Iterate over key-value pairs similar to vectors
    for (key, value) in &map {
        println!("{key}: {value}"); // Favourite colour: Blue
    }

    // error below because variables moved into map

    // println!("{field_name} : {field_value}"); // not variable not valid

    // move occurs because `field_value` has type `String`,
    // which does not implement the `Copy` trait

    // Inserting references in to hash map
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Red");

    let mut map2 = HashMap::new();

    // inserting references to variables
    map2.insert(&field_name, &field_value);

    for (key, value) in &map2 {
        println!("{key}: {value}"); // Favourite colour: Red
    }

    // still valid, as not moved, as reference was used
    println!("{field_name}: {field_value}");

    // Caution the values pointed to must be valid for as
    // long as the hash map is valid
    // see url:
    // https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
}
