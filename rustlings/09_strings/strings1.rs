// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    String::from("blue") //mf
}

// The function returns a String
// but "blue" is &str, fix with String::from 
// converts &str to String

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
