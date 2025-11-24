// https://doc.rust-lang.org/stable/book/ch06-02-match.html

enum Coin {
    Penny,
    Two_pence,
    Five_pence,
    Twenty_pence,
    Fifty_pence,
    One_pound,
    Two_pound,
}

fn value_in_pence(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Two_pence => 2,
        Coin::Five_pence => 5,
        Coin::Twenty_pence => 20,
        Coin::Fifty_pence => 50,
        Coin::One_pound => 100,
        Coin::Two_pound => 200,
        
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(us_coin: UsCoin) -> u8 {
    match us_coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        
    }
}

fn main() {
    println!("Value in pence for Penny: {}", { value_in_pence(Coin::Penny) });
    println!("Value in pence for Two_pound: {}", { value_in_pence(Coin::Two_pound) });

    value_in_cents(UsCoin::Dime);
    value_in_cents(UsCoin::Penny);
    value_in_cents(UsCoin::Quarter(UsState::Alaska));
    value_in_cents(UsCoin::Quarter(UsState::Alabama));
}
