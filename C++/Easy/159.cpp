/*
Daily Coding Problem: 159
Difficulty: Easy
Asked by: Google
Authored by: TenthEdict
*/

#include <iostream>
#include <unordered_set>

char first_recurring_char(const std::string& s){
    std::unordered_set<char> seen = {};

    for (char c : s) {
       if (seen.count(c)) {
        return c;
       } else {
        seen.insert(c);
       }
    }
    return '\0';
}

int main(){
    // char result = first_recurring_char("acbbac"); returns b
    char result = first_recurring_char("abcdef"); // returns null
    if (result != '\0') {
        std::cout << result << std::endl;
    } else {
        std::cout << "null" << std::endl;
    }
    return 0;
}

