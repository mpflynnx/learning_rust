// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // to do //
        }
    }
}

enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(us_coin: UsCoin) {
    let mut count = 0;
    match us_coin {
        UsCoin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1, // increment when coin not quarter
    }
    println!("non-quarter count: {count:?}");
}

// Functionally identical to above function uses if let else
fn value_in_cents_if_let(us_coin: UsCoin) {
    let mut count = 0;
    if let UsCoin::Quarter(state) = us_coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1; // increment when coin not quarter
    }
    println!("non-quarter count: {count:?}");
}

// let..else syntax
fn describe_state_quarter(us_coin: UsCoin) -> Option<String> {
    let UsCoin::Quarter(state) = us_coin else {
        return None; // default for non quarter coins
    };

    // Perform some computation when a value is present
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    value_in_cents(UsCoin::Penny);
    value_in_cents(UsCoin::Dime);
    value_in_cents(UsCoin::Quarter(UsState::Alaska));
    value_in_cents(UsCoin::Quarter(UsState::Alabama));

    value_in_cents_if_let(UsCoin::Quarter(UsState::Alaska));
    value_in_cents_if_let(UsCoin::Quarter(UsState::Alabama));

    // describe_state_quarter() usage
    // Common pattern to perform some computation when a value is present
    // and return a default value otherwise
    let result = describe_state_quarter(UsCoin::Quarter(UsState::Alaska));

    if let Some(description) = result {
        // This code block only executes if result is Some(String)
        println!("We found a quarter: {}", description);
    } else {
        // Optional else block for the None case
        println!("The coin was not a state quarter, so no description was generated.");
    }
}
