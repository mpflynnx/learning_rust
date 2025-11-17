fn main() {
    // loop {
    //     println!("Prints forever"); // ctrl c to quit
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");
}
