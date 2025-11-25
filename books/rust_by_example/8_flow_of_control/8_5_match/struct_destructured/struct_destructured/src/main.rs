struct Foo {
    x: (u32, u32),
    y: u32,
}

// Destructuring works with nested structs as well:
struct Bar {
    foo: Foo,
}

fn main() {
    let faa = Foo { x: (1, 2), y: 3 };

    // Destructure the Foo instance found in faa
    // into separate values using pattern matching
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}"); // Outside: x0 = (1, 2), y0 = 3

    let bar = Bar { foo: faa };

    // Destructure the Bar instance found in bar
    // into separate values using pattern matching
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
    // Nested: nested_x = (1, 2), nested_y = 3
}
