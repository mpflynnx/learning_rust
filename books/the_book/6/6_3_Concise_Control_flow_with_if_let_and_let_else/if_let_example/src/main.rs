// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

fn main() {
    let config_max = Some(3u8);

    // Only one condition needs to be true
    // but still have to cover-all conditions
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}."),
        _ => (), // cover-all conditions boilerplate
    }

    // Same as above but less boilerplate code
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}.")
    }
}
