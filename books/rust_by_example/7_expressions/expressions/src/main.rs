/*
fn main() {
    // statement
    // statement
    // statement
}
*/

/*
fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
}
 */

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x; // remove semi colon
    };

    println!("x is {:?}", x); // x is 5
    println!("y is {:?}", y); // y is 155
    println!("z is {:?}", z); // with ; at line34: z is () without: z is 10
}
