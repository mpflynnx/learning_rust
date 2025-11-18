// A tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Pair(i32, f32);

fn main() {
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    let black = Color(0, 0, 0);
    let origin = Point(45, 32, 21);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Point(x, y, z) = origin;
    let Pair(integer, decimal) = pair;

    println!("x:{:?} y:{:?} z:{:?}", x, y, z);

    println!("pair contains {:?} and {:?}", integer, decimal);
}
