use std::collections::HashMap;

fn main() {
    // .insert()
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // Same key, different value. Overwrites

    println!("{scores:?}"); // {"Blue": 20} new value 20

    // Insert if key doesn't already exist using
    // .entry() and .or_insert()
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    // no key "Yellow" exists so create it with value 50
    scores2.entry(String::from("Yellow")).or_insert(50);
    // key "Blue" does exist so leave unchanged.
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{scores2:?}"); // {"Blue": 10, "Yellow": 50}

    // counting occurrences of words
    let text = "hello wonderful world hello";

    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map3:?}"); // {"wonderful": 1, "world": 1, "hello": 2}
}
