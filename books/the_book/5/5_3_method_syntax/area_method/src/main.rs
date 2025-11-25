#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl for implementation block
// All functions defined within an impl block are called
// associated functions
// https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions

impl Rectangle {
    // Everything in this block is associated with Rectangle struct
    fn area(&self) -> u32 {
        // &self equivalent to self: &Self
        // &self parameter borrows the instance, allowing the
        // method to read its data without taking ownership,
        // and preventing modification.
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // method takes multiple parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that aren’t methods are often used for
    // constructors that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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

    // Associated functions are called by using the struct 
    // name followed by the double colon '::' and the function name.
    let rect4 = Rectangle::square(30);

    println!("rect1 holds rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 holds rect23 {}", rect1.can_hold(&rect3));
}
