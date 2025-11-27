
fn fix_incorrect_order() {
    cook_order();

    // super keyword needed to access function in src/lib.rs
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
