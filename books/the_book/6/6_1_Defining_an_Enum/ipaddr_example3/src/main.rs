// Demonstration ONLY Standard Library example

// Similar to Standard Lib Ipv4Addr struct:
// https://doc.rust-lang.org/stable/src/core/net/ip_addr.rs.html#73
struct Ipv4Addr {
    octets: [u8; 4],
}

impl Ipv4Addr {
    fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr {
            octets: [a, b, c, d],
        }
    }
}

// Similar to Standard Lib Ipv6Addr struct:
struct Ipv6Addr {
    octets: [u16; 8],
}

// https://doc.rust-lang.org/stable/src/core/net/ip_addr.rs.html#1297
impl Ipv6Addr {
    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        Ipv6Addr {
            octets: [a, b, c, d, e, f, g, h],
        }
    }
}

// Similar to Standard Lib IpAddr Enum:
// https://doc.rust-lang.org/stable/src/core/net/ip_addr.rs.html#30
enum IpAddrKind {
    V4(Ipv4Addr), // Takes a Struct
    V6(Ipv6Addr), // Takes a Struct
}

fn main() {
    // Of type IpAddrKind V4 variant
    // Uses V4 variants Structs new method
    let localhost_v4 = IpAddrKind::V4(Ipv4Addr::new(127, 0, 0, 1));

    // Of type IpAddrKind V6 variant
    // Uses V4 variants Structs new method
    let localhost_v6 = IpAddrKind::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
}
