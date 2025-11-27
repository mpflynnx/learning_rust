// hosting a sub-module of front_of_house
pub mod hosting; // compiler looks for file src/front_of_house/hosting.rs

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
