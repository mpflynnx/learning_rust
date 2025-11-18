#[derive(Debug)] // Must opt in to debug printing for this struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // create a
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // can now use rect1 again as borrowed by area()
    println!("rect1: {:?}", rect1); // one liner struct print
    // Output:
    // rect1: Rectangle { width: 30, height: 50 }

    println!("rect1: {rect1:#?}"); // pretty multi line struct
    // Output:
    // rect1: Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    // dbg! macro usage
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // Output: [src/main.rs:37:16] 30 * scale = 60
        height: 50,
    };

    dbg!(&rect2);
    // Output:
    // [src/main.rs:41:5] &rect2 = Rectangle {
    // width: 60,
    // height: 50,
}
