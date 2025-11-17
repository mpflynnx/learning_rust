#include <string>
#include <iostream>

bool is_space(char letter) {
    return std::string(" ").find(tolower(letter)) != std::string::npos;
    }

std::string first_word(const std::string& s) {

    std::string first_word;

    for (size_t i {0}; i < s.length(); i++) {
            if (is_space(s.at(i))) {
                i++; // skip space
                first_word = {s, i};
                break;
            }
        }
    

    return first_word;

}

int main()
{

    std::string s = "helloworld hello";

    std::cout << first_word(s) << "\n";

    return 0;

}
