fn main() {
    let x = 2.0; // f64 by default
    let y: f32 = 3.0;
    let y2 = 3.0_f32;
    let z: f64 = (x + y).into(); // you can convert an `f32` to an `f64`

    println!("z: {z}");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x} y:{y} z:{z}");
    println!("y: {0} z:{1} x:{2}", tup.1, tup.2, tup.0);

    // Arrays
    let array = [1, 2, 3, 4, 5];
    let months = ["jan", "feb", "mar"];
    println!("Debug trait print:\n{:?}", array);

    println!("Pretty printed:\n{:#?}", months);

    let a: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Debug trait print:\n{:?}", a);
    let b = [3; 5]; // Array of size 5 containing all 3
    println!("Debug trait print:\n{:?}", b);

    println!("{}", a[8]);

    // compile time out of bounds check
    //println!("{} {}", a[3-1], a[11-1]);
    //                          ^^^^^^^ index out of bounds: the length is 10 but the index is 10
}
