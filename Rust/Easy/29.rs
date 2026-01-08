// Daily Coding Problem: 29
// Difficulty: Easy
// Asked by: Amazon
// Author: TenthEdict

fn encode(s: &str) -> String {
    if s.len() == 0 {return String::from("")}

    let mut result: String = String::new();
    let mut count: i32 = 1;
    let mut current_char: char = s.chars().next().unwrap();

    for c in s.chars().skip(1) {
        if c == current_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current_char);
            count = 1;
            current_char = c;
        }
    }
    result.push_str(&count.to_string());
    result.push(current_char);
    result
}

fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut count: u32 = 0;
    
    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            count = count * 10 + digit;
        } else {
            result.push_str(&c.to_string().repeat(count as usize));
            count = 0;
        }
    }
    result
}

fn main() {
    // Basic test
    println!("{}", encode("AAAABBBCCDAA")); // 4A3B2C1D2A
    println!("{}", decode("4A3B2C1D2A"));   // AAAABBBCCDAA
    
    // Round-trip test
    println!("{}", decode(&encode("AAAABBBCCDAA"))); // AAAABBBCCDAA
    
    // Edge cases
    println!("{}", encode("A"));            // Single char
    println!("{}", encode("ABCDEF"));       // No repeats
    println!("{}", decode("12A"));          // Multi-digit count (12 A's)
    println!("{}", encode(""));             // Empty string
}
