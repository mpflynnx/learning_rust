#include <string>
#include <iostream>

// Rust code:
// Reference borrowing using &, does not own 
// Note: no need for &mut as function doesn't mutate s

// s is immutable (const) by default in Rust
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// C++ code
std::size_t calculate_length(const std::string &s){ // const as not modifying
    return s.length();
}

// Rust code: 
// To modify a borrowed value must use &mut
// Note: must use &mut if function will mutate s:
// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// C++ code
std::string change(std::string &s){ // no const as modifying s
    return s.append(", world");
}

int main()
{

    std::string s1 = "hello";
    auto len = calculate_length(s1);

    std::cout << "The length of '" << s1 << "' is " << len << "." << std::endl;

    // No need for & in C++ function call
    change(s1); 
    
    // Rust code: must use &mut to match &mut in function
    // change(&mut s1);

    len = calculate_length(s1);

    std::cout << "The length of '" << s1 << "' is " << len << "." << std::endl;


    /* Mutable references */

    // C++ creating more than one mutable reference to s1
    // In Rust this is not allowed, compile error
    // std::string &r1 = s1; // One is Okay
    // std::string &r2 = s1; // Opps C++ allows this, can cause UB

    // C++ use const for safety, no problems now
    std::string const &r1 = s1; // r1 cannot mutate s1
    std::string const &r2 = s1; // r2 cannot mutate s1

    std::cout << "r1: " << r1 << "\n"; // r1: hello, world
    std::cout << "r2: " << r2 << "\n"; // r2: hello, world
    
    // C++ mutable reference Problem
    std::string &r3 = s1; //BIG PROBLEM allowed in C++ not Rust
    // In Rust cannot have a mutable reference if there are already 
    // immutable references r1 and r2 in scope
    // in Rust r1 and r2 would go out of scope after: println!("{r1} and {r2}");
    // Rust compiler checks that r1 and r2 are not used again.

    r3.append("... again"); // OPPS! mutates s1, r1 and r2
    // Only a problem, if r1 and r2 are needed again
    // C++ allows this at compile time, Rust doesn't
    std::cout << "r1: " << r1 << "\n"; // r1: hello, world... again
    std::cout << "r2: " << r2 << "\n"; // r2: hello, world... again
    std::cout << "r3: " << r3 << "\n"; // r3: hello, world... again

    /*
    // Solution: use r1 and r2, then end the scope, before r3 is needed
    { // Start a new scope
        std::string const &r1 = s1; // r1 is created
        std::string const &r2 = s1; // r2 is created

        std::cout << "r1: " << r1 << "\n"; // r1: hello, world
        std::cout << "r2: " << r2 << "\n"; // r2: hello, world

    } // End of scope. r1 and r2 are automatically destroyed (cease to exist).
      // s1 remains valid.
    */

    return 0;

}
