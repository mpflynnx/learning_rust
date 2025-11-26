// if let can be used to match any enum value:

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("'a' matched Foo::Bar");
    }

    // b matches Foo:Baz so nothing is printed after
    // below
    if let Foo::Bar = b {
        println!("'b' matched Foo::Bar");
    }

    if let Foo::Qux(i) = c {
        println!("'c' is: {}", i)
    }

    // Binding also works with `if let`
    if let Foo::Qux(i @ 100) = c {
        println!("'c' is one hundred");
    }
}
