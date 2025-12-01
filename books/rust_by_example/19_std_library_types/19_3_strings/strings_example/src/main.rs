fn main() {
    // &str is a slice (&[u8]) that always points to a valid UTF-8 sequence,
    // and can be used to view into a String, just like &[T] is a view into Vec<T>

    // examples
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory

    // let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    // below is same as above without type annotations
    let pangram = "the quick brown fox jumps over the lazy dog"; //type inferred as: &'static str

    // println!("Pangram: {}", pangram);
    // below is same as above
    println!("Pangram: {pangram}");

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("chars: {:?}", chars); // Use `{:?}` (or {:#?} for pretty-print) instead of {}
                                    // chars: [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']

    // Strings
    // A String is stored as a vector of bytes (Vec<u8>), but guaranteed
    // to always be a valid UTF-8 sequence. String is heap allocated,
    // growable and not null terminated.

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    println!("string: {}", string);
    // string:  , a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
    // Used characters: a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
