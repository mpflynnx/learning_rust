fn main() {
    // Create a new empty vector to hold only type i32
    let v1: Vec<i32> = Vec::new();

    // Create a new vector containing initial i32 type values
    // vec! macro usage, type inferred as i32
    let v2 = vec![123, 34, 78, 34]; // let v2 = vec![123i32, 34, 78, 34]; forced infer with 123i32

    // Iterating over the value of a Vector.
    for i in &v2 {
        println!("{i}");
    }

    // Updating the contents of a vector using 'push' method
    let mut v3 = Vec::new(); // must follow this with a v3.push()
    v3.push(45); // compiler infers i32 type here
    v3.push(65);
    // v3.push(92);
    // v3.push(12);

    // Reading elements of a vector, no empty check!

    // let third_element: &i32 = &v3[2];
    // println!("The third element is {third_element}.");

    // panic error is no value at element 2
    // thread 'main' (11798) panicked at src/main.rs:17:34:
    // index out of bounds: the len is 2 but the index is 2

    // Reading elements of a vector using .get which return Option<T>
    let third_element: Option<&i32> = v3.get(2);
    match third_element {
        Some(third_element) => println!("The third element is {third_element}."),
        None => println!("There is no third element."),
    }

    // Add two more values to the mutable v3 vector
    v3.push(92);
    v3.push(12);

    // iterate over a mutable vector to update the elements
    // *i deference the pointer to get the value then perform addition
    for i in &mut v3 {
        *i += 50; // add 50 to the value at each element
        println!("{i}");
    }

    // Defining an enum to store values of different types in one vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(456),
        SpreadsheetCell::Text(String::from("orange")),
        SpreadsheetCell::Float(3.142),
    ];
}
