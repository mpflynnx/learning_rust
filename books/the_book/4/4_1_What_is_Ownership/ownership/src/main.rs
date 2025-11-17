fn main() {
    
    // string literal sl is not valid here

    {   // sl is not valid here
        
        let sl = "Hello"; // sl is now valid (in scope)

        // do something with sl
        // sl size is known at compile time, placed on the stack
        
        println!("{sl}"); // prints `Hello`

    
    }   // sl is no longer valid (out of scope)

    { // s is not valid here

        let mut s = String::from("Hello"); // s is now valid (in scope)

        s.push_str(", world!"); // mutates s,  appends ", world" to s
        
        println!("{s}"); // prints `Hello, world!`
    } // s is not valid here, Rust calls String private drop() which frees memory

    {
        let s1 = String::from("Hello");
        let s2 = s1; // s1 `moved` to s2, s1 out of scope now

        // println!("{s1}, world!"); // Error s1 out of scope
        println!("{s2}, world!")
    } // s2 out of scope now

    { // Scope and assignment

        let mut s = String::from("hello");
        s = String::from("yello"); // original s out of scope, new s created here

        println!("{s}");
    
    } // s out of scope here


    { // Clone, duplicates on the heap

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1:{s1}, s2:{s2}");

    }

}
