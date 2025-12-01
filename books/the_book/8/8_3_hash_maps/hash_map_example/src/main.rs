use std::collections::HashMap; // needed for Hash maps

fn main() {
    // Create a new empty hash map
    let mut scores = HashMap::new(); //  no macro like vec! for hash maps

    scores.insert(String::from("Blues"), 10); // inferred String and i32 types
    scores.insert(String::from("Yellows"), 50);

    // Accessing values in a Hash Map
    let mut team_name = String::from("Blues");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} score: {score}"); // Blues score: 10

    team_name = String::from("Yellows");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} score: {score}"); // Blues score: 10

    // Iterate over key-value pairs similar to vectors
    for (key, value) in &scores {
        println!("{key}: {value}"); // Blues: 10
                                    // Yellows: 50
    }
}
