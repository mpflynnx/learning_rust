// https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html

// An enum called Option<T> in the std library is used when
// absence is a possibility

// You do not need to define the Option<T> enum

// https://doc.rust-lang.org/stable/src/core/option.rs.html#594
// pub enum Option<T> {
//     None, // No value
//     Some(T), // Can hold one piece of data of any type T
// }

// There are many implementation methods for Option<T>

// https://doc.rust-lang.org/stable/src/core/option.rs.html#629
// impl<T> Option<T> {
//      fn is_some(&self) -> bool

// ....
//}
// is_some
// Returns `true` if the option is a [`Some`] and the value
// inside of it matches a predicate.
// https://doc.rust-lang.org/stable/src/core/option.rs.html#635

fn main() {
    // Do not need 'Option::' prefix, but shown as it is
    // used like a normal enum

    // The type can be inferred from Some(5) as i32 so Option<i32>
    // is also not required.

    // long form below:
    let some_number: Option<i32> = Option::Some(5);

    //short form below
    // let some_number = Some(5);
    // using a Option<T> method is_some()
    assert_eq!(some_number.is_some(), true);

    // Compiler cannot infer type from None
    // So must declared a type as Option<i32>
    let absent_number: Option<i32> = None;
    assert_eq!(absent_number.is_some(), false);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    assert_eq!(six, Some(6));
    assert_eq!(none, None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
