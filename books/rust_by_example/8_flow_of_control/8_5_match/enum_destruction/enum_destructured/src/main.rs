// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Colour {
    // These 3 are specified solely by their name.
    Red,
    Green,
    Blue,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let colour = Colour::RGB(132, 15, 59);

    println!("What colour is it?");

    // Destructured enum using a match
    match colour {
        Colour::Red => println!("The colour is Red!"),
        Colour::Green => println!("The colour is Green!"),
        Colour::Blue => println!("The colour is Blue!"),
        Colour::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Colour::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Colour::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Colour::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Colour::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
}
