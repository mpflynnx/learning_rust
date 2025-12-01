// Sometimes there are just too many characters that need to be escaped
// or it's just much more convenient to write a string out as-is. This
// is where raw string literals come into play.

fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str); // Escapes don't work here: \x3F \u{211D}

    // If you need quotes in a raw string, add a pair of r#" "" "#
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes); // And then I said: "There is no escape!"

    // If you need "# in your string, just use more #'s in the delimiter.
    // You can use up to 255 #'s.
    let longer_delimiter = r###"A string with "#" in it. And even "##!""###;
    println!("{}", longer_delimiter); // A string with "#" in it. And even "##!"
}
