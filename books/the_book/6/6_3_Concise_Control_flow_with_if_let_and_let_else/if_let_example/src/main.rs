// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

fn main() {

    // long form
    // let config_max: Option<u8> = Option::Some(3u8);

    // short form of above
    let config_max = Some(3u8); // Some is Option<t>

    // Only one condition needs to be true
    // but still have to cover-all conditions
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}."),
        _ => (), // cover-all conditions boilerplate
    }

    // Same as above but less boilerplate code
    // 'if let' is syntactic sugar for a 'match' with one
    // specific arm and a catch-all '_' arm, making it concise for
    // single-pattern matching.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}.")
    }
    // safely binds the inner value of the 'Some' variant to 'max'.
}
