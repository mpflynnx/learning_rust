fn main() {
    let s1 = gives_ownership(); // s1 receives function return value String

    println!("{s1}"); // "yours"

    let s2 = String::from("Hello");

    let s3 = take_and_give_back(s2); // s2 moved to function out of scope now

    // println!("{s2}"); s2 out of scope as moved to function

    println!("{s3}");
}

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string
}

fn take_and_give_back(a_string: String) -> String {

    a_string

}
