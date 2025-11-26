// nested modules
// define module with 'mod' keyword
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {} // access from within a module using super

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super equivalent to '..' in linux filesystem    }
    }
    fn cook_order() {}

    // Define struct public using pub keyword, fields remain private unless pub used
    pub struct Breakfast {
        pub toast: String,      // must define field public
        seasonal_fruit: String, // this remains private or immutable
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast), // mutable, as pub keyword used
                seasonal_fruit: String::from("blueberries"), // immutable
            }
        }
    }

    // Define enum public, all variants become public
    pub enum Appetizer {
        Soup,  // public by default
        Salad, // public by default
    }
}

use crate::front_of_house::hosting; // idiomatic bring functions parent into scope

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
