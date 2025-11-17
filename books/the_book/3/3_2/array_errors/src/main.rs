use std::io;

fn main() {
    let array = [2, 4, 6, 8, 10];

    println!("Pick a array index to view:");

    let mut user_response = String::new();

    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    let user_response: usize = user_response
        .trim()
        .parse()
        .expect("User must enter a number");

    let element = array[user_response];

    println!("The value of element at index {user_response} is: {element}");

    // if user enters 7 get runtime error as shown below
    // thread 'main' panicked at src/main.rs:19:19:
    // index out of bounds: the len is 5 but the index is 7
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
