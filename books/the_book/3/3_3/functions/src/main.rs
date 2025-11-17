fn main() {
    println!("Hello, world!");

    another_function(4);

    print_labeled_measurement(5, 'm');

    let y = {
        let x = 3;
        x + 1 // no semi-colon here it's an expression
    };

    println!("The value of y is: {y}");

    // Functions with return values
    let z = five();
    println!("The value of z is: {z}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("Another function.\n");
    println!("The value of x: {x}");
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("The measurement is {value}{label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
