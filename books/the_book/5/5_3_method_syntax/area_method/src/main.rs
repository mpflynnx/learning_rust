#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl for implementation block
impl Rectangle {
    // Everything in this block is associated with Rectangle struct
    fn area(&self) -> u32 {
        // &self equivalent to self: &Self
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // method takes multiple parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the Rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width())
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 holds rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 holds rect23 {}", rect1.can_hold(&rect3));
}
