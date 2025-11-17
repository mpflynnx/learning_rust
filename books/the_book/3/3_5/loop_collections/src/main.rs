fn main() {
    let a = [10, 20, 30, 40, 50];

    // Error prone example of looping a collection
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Use a for loop, more concise. less error prone
    for element in a {
        println!("the value is: {element}");
    }

    // Recommended: Use a for loop i place of a while loop as well
    for number in (1..=10).rev() {
        println!("{number}!");   
    }
    
    println!("LIFTOFF!!!");
}
