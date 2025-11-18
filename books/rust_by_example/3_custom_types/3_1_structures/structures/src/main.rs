// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Activity 1
// Add a function rect_area which calculates the area of a 
// Rectangle (try using nested destructuring).
fn rect_area(r: Rectangle) -> f32{
    r.top_left.y * r.bottom_right.x
}

// Activity 2
// Add a function square which takes a Point and a f32 as 
// arguments, and returns a Rectangle with its top left 
// corner on the point, and a width and height corresponding to the f32.
fn square(p: Point, f: f32) -> Rectangle {
    Rectangle{
        top_left: p,
        bottom_right: Point {x:f, y:f}
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    // let _rectangle = Rectangle {
    //     // struct instantiation is an expression too
    //     top_left: Point { x: left_edge, y: top_edge },
    //     bottom_right: bottom_right,
    // };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let new_rectangle = Rectangle{
        top_left: Point {x:0.0, y:5.0},
        bottom_right: Point {x:5.0, y:0.0}
    };

    // Activity 1
    println!("{:?}", rect_area(new_rectangle));

    // Activity 2
    println!("{:?}", square(Point {x:0.0, y:10.0}, 5.0));
}
