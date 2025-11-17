fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 1 {
                break; // break non labelled loop
            }

            if count == 6 {
                break 'counting_up; // break labelled loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");
}
