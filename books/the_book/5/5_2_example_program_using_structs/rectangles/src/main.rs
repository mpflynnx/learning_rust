struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: Rectangle) -> u32 {
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
        area(rect1)
    );

    // cannot use rect1 again as moved into area()
    // and destroyed when area completes 
    // println!("{:?}", rect1);
}
