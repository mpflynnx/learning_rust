fn main() {
    let s = String::from("Hello"); // s in scope here. String types have no copy traits
    println!("{s}"); // this is allowed as s in scope

    take_ownership(s); // s no longer available in this scope, s moves to function

    // println!("{s}"); // this is not allowed now due to s out of scope here

    let x = 5; // x of type default i32 now in scope

    makes_copy(x); // x still available after function 
    // as it's an i32 which implements copy trait

    println!("{x}"); // 5, x still available in this scope
}

fn take_ownership(some_string: String) {
    println!("{some_string}"); // "Hello"
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}") // 5
}
