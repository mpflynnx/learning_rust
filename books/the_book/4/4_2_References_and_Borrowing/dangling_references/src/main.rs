fn main() {

    // let reference_to_nothing = dangle();
    let reference_to_nothing = dangle_fix();

    println!("{reference_to_nothing}");

}

/// Return type is burrowed but no value to borrow it from
// fn dangle() -> &String { // returns a reference to a string
    
//     let s = String::from("Hello"); // s is new string

//     &s // return reference to s

// } // s goes out of scope here, so its memory goes away freed

fn dangle_fix() -> String { // return a String
    
    let s = String::from("Hello"); // s is new string

    s // return String

} // s is moved to caller of function, ownership is moved