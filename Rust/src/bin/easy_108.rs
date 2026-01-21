// Daily Coding Problem: 108
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

fn check_string_rotation(a: &str, b: &str) -> bool {
    if a.len() != b.len() { return false }
    let doubled = a.repeat(2);
    doubled.contains(b)
}

fn main() {
    // Should be true
    println!("abcde, cdeab: {}", check_string_rotation("abcde", "cdeab")); // true
    println!("abcde, abcde: {}", check_string_rotation("abcde", "abcde")); // true (0 rotations)
    
    // Should be false
    println!("abc, acb: {}", check_string_rotation("abc", "acb"));   // false
    println!("abc, abcd: {}", check_string_rotation("abc", "abcd")); // false (different lengths)
    
    // Edge cases
    println!("'', '': {}", check_string_rotation("", ""));           // what do you expect?
    println!("a, a: {}", check_string_rotation("a", "a"));           // true
}
