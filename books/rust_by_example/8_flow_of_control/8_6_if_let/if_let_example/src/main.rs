fn main() {
    let number = Some(8);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("It matched to {:?}.", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("It matched to {:?}.", i);
    } else {
        println!("It match a number, perhaps a letter?");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("It matched to {:?}.", i);
        // Destructure failed. Evaluate an `else if` condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("It didn't match a number, perhaps a letter?");
    } else {
        // the condition evaluated to false
        println!("It didn't match a letter, perhaps a emoticon?")
    }
}
