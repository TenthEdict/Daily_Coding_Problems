/*
Daily Coding Problem: 108
Difficulty: Easy
Asked by: Google
Authored by: TenthEdict
*/

#include <iostream>
#include <string>

bool check_string_rotation(const std::string& a, const std::string& b){
    if (a.length() != b.length())
    {
        return false;
    }

    std::string doubled = a + a;

    bool result = doubled.find(b) != std::string::npos;
    
    return result;
}

int main(){

    std::string a = "abcdef";
    std::string b = "bcdefa";

    bool result = check_string_rotation(a, b);

    if (result) {
        std::cout << "A can rotate to match B" << std::endl;
    } else {
        std::cout << "A cannot rotate to match B" << std::endl;
    }
    return 0;
}
