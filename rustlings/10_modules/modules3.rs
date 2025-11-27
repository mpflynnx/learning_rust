// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// mf: SystemTime is a struct
// mf: duration_since is a function of SystemTime
// mf: UNIX_EPOCH is a constant from std::time

// mf: 1st attempt: brings all public items from std::time
// use std::time::*; // 1st attempt: brings all public items from std::time

// mf: 2nd attempt, on two lines, ignores duration_since function
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

// mf: 3rd attempt, best solution
use std::time::{UNIX_EPOCH, SystemTime}; // ignores 'duration_since' as this is a function of 'SystemTime'
fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
