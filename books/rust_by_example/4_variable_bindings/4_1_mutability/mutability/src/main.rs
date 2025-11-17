fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding); // Before mutation: 1

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding); // After mutation: 2

    // Error! Cannot assign a new value to an immutable variable
    //_immutable_binding += 1;
}
