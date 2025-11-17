fn main() {
    let s = String::from("Hello World");
    let first_word = head(&s);
    println!("Head of s: {first_word}");
    let second_word = tail(&s);
    println!("Tail of s: {second_word}");
}

fn head(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over array using iter()
                                            // enumerate() wraps the result as a tuple
                                            // tuple is index i and reference &item to element
        if item == b' ' { // byte literal syntax b' '
            return &s[..i]; // return position in array (index)
        }
    }

    &s[..]
}

fn tail(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over array using iter()
                                            // enumerate() wraps the result as a tuple
                                            // tuple is index i and reference &item to element
        if item == b' ' { // byte literal syntax b' '
            return &s[i+1..]; // return position in array (index + 1) ignore ' '
        }
    }

    &s[..]
}
