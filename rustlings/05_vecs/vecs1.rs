fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact 
    // same elements as in the array `a`.
    
    // Use the vector macro.
    // let v = ???;
    // mf:
    // let v = vec![10,20,30,40];

    // mf alternative using 'to_vec()'
    // For basic types like i32, to_vec() is generally preferred 
    // for its simplicity, as it is a direct and clear operation 
    // for creating a vector from an array.
    // let v: Vec<i32> = a.to_vec();

    // mf: alternative
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // Convert the array into an iterator and collect into a Vec
    let v: Vec<i32> = a.into_iter().collect();
    // Another common pattern in Rust is to convert the array 
    // into an iterator and then use the collect() method to 
    // gather the results into a new Vec. For arrays, into_iter() 
    // will iterate over the elements by value (copying them, 
    // if possible, for types that implement Copy).
    

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
