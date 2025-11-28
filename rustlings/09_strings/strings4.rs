// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {

    // String literals i.e "blue" are stored in the program’s binary and are 
    // therefore string slices.
    string_slice("blue");

    // we use the to_string method, which is available on any type 
    // that implements the Display trait, as string literals do
    string("red".to_string()); 

    // We can also use the function String::from to create a String 
    // from a string literal.
    string(String::from("hi"));

    // to_owned() returns a String
    string("rust is fun!".to_owned());

    // into() converts input &str into output 
    string("nice weather".into());

    // format! returns a string
    string (format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.

    // mf: use [] with a range to create a string slice containing 
    // particular bytes
    string_slice (&String::from("abc")[0..1]);

    // trim() returns a string slice
    string_slice ("  hello there ".trim());

    // replace() returns a string
    string("Happy Monday!".replace("Mon", "Tues"));

    // to_lowercase() returns string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
