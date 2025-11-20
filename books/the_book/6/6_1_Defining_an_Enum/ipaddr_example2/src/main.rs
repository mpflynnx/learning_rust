// enum is a custom type which can have multiple variants
// but only one variant can be defined
enum IpAddrKind {
    V4(u8, u8, u8, u8), // V4 variant holds four u8 types type
    V6(String),         // V6 variant holds String type
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1); // variant V4

    // six is a IpAddrKind type of variant V6
    let loopback = IpAddrKind::V6(String::from("::1")); // Variant V6
}
