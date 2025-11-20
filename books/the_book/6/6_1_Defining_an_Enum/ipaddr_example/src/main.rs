// enum is a custom type which can have multiple variants
// but only one variant can be defined
enum IpAddrKind {
    V4, // V4 variant
    V6, // V6 variant
}

// A function that takes an IpAddrKind type as a parameter
fn route(ip_kind: IpAddrKind) {}

fn main() {
    // four is a IpAddrKind type of variant V4
    let four = IpAddrKind::V4; // variant V4

    // six is a IpAddrKind type of variant V6
    let six = IpAddrKind::V6; // Variant V6

    // Use Enum variant in function call
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
