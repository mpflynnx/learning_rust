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
    // let some_number = Option::Some(5);
    // short form below
    let some_number = Some(5);
    // using a Option<T> method is_some()
    assert_eq!(some_number.is_some(), true);

    let absent_number: Option<i32> = None;
    assert_eq!(absent_number.is_some(), false);

}
