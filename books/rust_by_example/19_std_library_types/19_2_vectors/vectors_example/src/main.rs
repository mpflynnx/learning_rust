fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into {:?}", collected_iterator);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);

    // The vec! macro can be used to initialise a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs); // Initial vector: [1, 2, 3]

    // Insert new element at the end of the vector
    println!("Push 45 into the vector");
    xs.push(45);
    println!("Vector: {:?}", xs); // Vector: [1, 2, 3, 45]

    // The `len` method yields the number of elements currently stored in a vector
    println!("Vector length: {}", collected_iterator.len()); // Vector length: 10
    println!("Vector length: {}", xs.len()); // Vector length: 4

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());
    println!("Vector: {:?}", xs); // Vector: [1, 2, 3]

    // Out of bounds indexing yields a panic
    // println!("Fourth element: {}", xs[3]); // index out of bounds: the len is 3 but the index is 3
    // FIXME ^ Comment out this line

    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs); // Updated vector: [3, 6, 9]
}
