// https://doc.rust-lang.org/stable/book/ch06-02-match.html

// The mandatory exhaustive checking by the compiler is the main power of
// 'match', ensuring no possible value is missed.
fn main() {
    let dice_roll = 9;
    // match is exhaustive, must match all possible cases
    // use _ to catch-all other cases, rather than define them all
    match dice_roll {
        3 => match_for_3(),
        7 => match_for_7(),
        _ => match_for_any_other_number(),
    }
}

fn match_for_3() {}
fn match_for_7() {}
fn match_for_any_other_number() {}
