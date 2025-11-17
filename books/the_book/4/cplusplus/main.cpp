/*
 * File: main.cpp
 * --------------
 * Blank C++ project configured to use Stanford cslib and Qt
 */

// #include "console.h"
#include "simpio.h"
#include <string>
#include <iostream>

void take_ownership(const std::string some_string){
    std::cout << "some_string passed to function: " << some_string << std::endl;
}

std::string gives_ownership(){
    std::string some_string = "yours";
    return some_string;
}

std::string take_and_give_back(std::string a_string) {
    return a_string;
}

int main()
{


    // string name = getLine("What is your name?");
    // std::cout << "Hello, " << name << std::endl;


    /* */
    {
    char sl[] = "Hello, I'm on the stack"; // string literal size known at compile time
        std::cout << sl << "\n";
    } // sl out of scope

    // std::cout << sl << "\n"; // fails to compile, sl out of scope


    /* C++ Deep copy heap data */
    {
    std::string s0;
    s0 = "C++ Rocks!"; // s0 grows dynamically as needed

    std::string s01 = s0; // Deep Copies (Duplicates memory) s0 into s01, Known as clone in Rust: let s01 = s0.clone()

    s0.at(9) = '?'; // replace ! with ? at index 9
    std::cout << s0 << '\n'; // Updated: C++ Rocks?
    std::cout << s01 << '\n'; // Original: C++ Rocks!
    }

    // std::cout << s01 << '\n'; // fails to compile, s01 out of scope


    /* */
    std::string const s1 = "Frank"; // In Rust immutable by default, In C++ need to add const for immutability
    std::string const s2 = s1; // In c++ a deep copy. In Rust: s1 `moved` to s2, s1 out of scope now in Rust

    std::cout << "s1 still in scope in C++" << s1 << std::endl;

    take_ownership(s2); // In Rust s2 now no longer in scope, s2 moved to function

    std::cout << s2 << std::endl; // In Rust this is not allowed now due to s2 out of scope here

    /* return values */
    std::string s3 = gives_ownership();
    std::cout << s3 << std::endl;

    std::string s4 = "Hello";

    std::string s5 = take_and_give_back(s4);

    std::cout << s4 << std::endl; // In Rust s4 is out of scope here as moved to function

    std::cout << s5 << std::endl;

    /* reference scope */
    std::string s6 = "Hello";

    auto const r1 = &s6; // immutable reference to s6
    auto const r2 = &s6; // immutable reference to s6

    std::cout << *r1 << " " << *r2 << "\n"; //dereference r1 and r2

    auto const r3 = &s6; // In Rust, if r1 and r2 are used again then error here
        // cannot borrow `s6` as mutable because it is also borrowed as immutable


    std::cout << *r1 << " " << *r2 << "\n"; // Fine in c++

    std::cout << *r3 << "\n";

    return 0;
}
