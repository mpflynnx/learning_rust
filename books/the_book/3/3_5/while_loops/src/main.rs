fn main() {
    let mut number = 10;

    // Not the best rust way of doing this
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Recommended: Use a for loop
    for number in (1..=10).rev() {
        println!("{number}!");   
    }
    
    println!("LIFTOFF!!!");
}
