fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");
    println!("{r1} and {r2}");

    let r3 = &mut s; // This is fine if r1 and r2 never used again
                     // If r1 and r2 are used again then error here
                     // cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("{r1} and {r2}"); // don't use r1 and r2 again if you want to use r3
    println!("{r3}");
}
