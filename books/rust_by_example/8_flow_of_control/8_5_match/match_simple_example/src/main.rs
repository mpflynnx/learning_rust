fn main() {
    let number = 7;

    println!("Tell me about {}.", number);
    match number {
        // number is i32 type, maximum is a big number, cannot check every number
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number."),
        13..=19 => println!("It's in the teens."),
        _ => println!("Ain't nothing special."), // catch-all needed here
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} == {}", boolean, binary);
}
