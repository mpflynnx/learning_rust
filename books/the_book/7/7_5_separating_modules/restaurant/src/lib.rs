// This file is the crate root in this example. src/lib.rs
// same use of modules applies to src/main.rs

mod front_of_house; // compiler looks for file src/front_of_house.rs
pub use crate::front_of_house::hosting; // idiomatic bring functions parent into scope

fn deliver_order() {} // access from within a modules using super

mod back_of_house; // compiler looks for file src/back_of_house.rs

pub fn eat_at_restaurant() {
    // Absolute path, if no 'use' keyword
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, if no 'use' keyword
    front_of_house::hosting::add_to_waitlist();

    // When 'use' keyword used above
    // i.e use crate::front_of_house::hosting;
    hosting::add_to_waitlist(); // idiomatic: we know 'add_to_waitlist()' comes from hosting

    // Struct usage example
    // Order a breakfast in summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Update order meal struct for different toast
    meal.toast = String::from("Granary");
    // meal.seasonal_fruit = String::from("peaches"); // private field error
    println!("I want {} toast now", meal.toast);

    // Enum usage example
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
