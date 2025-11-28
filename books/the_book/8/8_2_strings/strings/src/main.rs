fn main() {
    // new empty string
    // String is a wrapper over a Vec<u8>
    let mut s1 = String::new(); // new empty string

    // using to_string() to populate new s2 with data
    let data = "here is some data";
    let s2 = data.to_string();

    // use the literal directly with to_string()
    let s3 = "here is some data".to_string();
    // or use String::from
    let s4 = String::from("Here is some data");

    // String are UTF-8 encoded, can use different languages
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");

    // appending a string
    let mut s5 = String::from("some more data");
    println!("{s5}"); // "some more data"
    s5.push_str(" to store for your project"); // append a string slice
    println!("{s5}"); // "some more data to store for your project"

    // add a single character using push
    let mut s6 = String::from("Whats the missing char");
    s6.push('?');
    println!("{s6}"); // "Whats the missing char?"

    // format macro usage
    let naughts = String::from("naughts");
    let and = String::from("and");
    let crosses = String::from("crosses");
    // Very readable less prune to errors, retains access to original
    let sentence = format!("{naughts} {and} {crosses}"); // uses references
    println!("{sentence}");
    println!("{naughts}"); // still valid
    println!("{and}"); //  still valid
    println!("{crosses}"); // still valid

    // + usage, must use &
    let naughts = String::from("naughts");
    let and = String::from("and");
    let crosses = String::from("crosses");
    // No very readable prune to errors, loses access to naughts
    let sentence = naughts + " " + &and + " " + &crosses;
    println!("{sentence}");
    // println!("{naughts}"); // not valid now
    println!("{and}"); //  still valid
    println!("{crosses}"); // still valid

    // Rust Strings don't support indexing to prevent bugs with
    // non-english language words.

    // iterating over a UTF-8 non english string
    let hello = String::from("Здравствуйте");

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
