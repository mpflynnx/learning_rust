fn main() {
    let number = 7;

    if number < 5 {
        println!("True");
    } else {
        println!("False")
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("number: {number}"); // 5

    let condition = false;
    let number = if condition {5} else {6};
    println!("number: {number}"); // 6

}
