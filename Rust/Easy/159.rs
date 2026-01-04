// Daily Coding Problem: 159
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

use std::collections::HashSet;

fn first_recurring_char(s: &str) -> Option<char> {
    // 1. Initialize an empty HashSet to track seen characters
    let mut hset = HashSet::new();
    // 2. Iterate through each character in the string
    for c in s.chars() {
        // 3. If we've seen this character before, return it
        if hset.contains(&c) {
            return Some(c);
        } else {
            // 4. Otherwise, add it to our seen set
            hset.insert(c);
        };
    }
    // 5. If we get through the whole string, return None
    return None;
}

fn main() {
    let repeat = "abcbde";
    let norepeat = "abcdef";
    println!("{:?}", first_recurring_char(repeat));
    println!("{:?}", first_recurring_char(norepeat));
}
