fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.

    // https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned
    // Creates owned data from borrowed data, usually by cloning.
    // input is &str to concatenate we create a new String first.
    
    // mf:
    // input.to_owned() + " world!" 
    
    // mf: alt
    // let mut s = input.to_owned(); // creates mut s from input &str
    // s.push_str(" world!"); // appends string slice to s
    // s

    // The macro `format!` has the same syntax as `println!`, but it returns a
    // string instead of printing it to the terminal.
    // Equivalent to `input.to_string() + " world!"`
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
