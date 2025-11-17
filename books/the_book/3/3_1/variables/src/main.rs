const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5; // (gdb) x &x 0x7fffffffdb64: 0x00000005

    println!("x: {x}"); // (gdb) p x $2 = 6

    let x = x + 1; // (gdb) x &x 0x7fffffffdbb8:	0x00000006

    {
        let x = x * 2; // (gdb) x &x 0x7fffffffdbbc:	0x0000000c
        println!("inner x: {x}"); // (gdb) p x $3 = 12
    }

    println!("x: {x}"); // 0x7fffffffdbb8:	0x00000006

    println!(
        "The numbers of 
    seconds in 3 hours is {THREE_HOURS_IN_SECONDS}"
    );

    let spaces = "    "; // (gdb) x &spaces 0x7fffffffdcb0:	0x5555b05c

    println!("spaces: {spaces}");
    let spaces = spaces.len(); // (gdb) x &spaces 0x7fffffffdd10:	0x00000004

    println!("shadowed spaces: {spaces}");
}
