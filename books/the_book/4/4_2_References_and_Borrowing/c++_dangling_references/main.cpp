#include <string>
#include <iostream>

/*
// Bad code, returns reference
// Compile warning: reference to local variable ‘s’ returned [-Wreturn-local-addr]

std::string& dangle(){
    std::string s = "hello";
    return s;
} // s goes out of scope here

*/

// Return string not reference
std::string dangle_fix(){
    std::string s = "hello";
    return s;
} // s is returned


int main()
{

    // Bad code!

    // std::string r1 = dangle(); // bad code

    // In C++ this compiles and runs and crashes
    // terminated by signal SIGSEGV (Address boundary error)

    std::string r2 = dangle_fix();
    std::cout << r2 << "\n";

    return 0;

}
