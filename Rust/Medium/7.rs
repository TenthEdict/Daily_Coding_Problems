// Daily Coding Problem: 7
// Difficulty: Medium
// Asked by: Facebook
// Author: TenthEdict

fn count_decodable(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut dp: Vec<i32> = vec![0; n + 1];
    
    dp[0] = 1;
    
    for i in 1..=n {
        // Check single digit (chars[i-1])
        if chars[i-1] != '0' {
            dp[i] += dp[i-1];
        }
        // Check two digits (chars[i-2] and chars[i-1])
        // TODO: if i >= 2 and valid, dp[i] += dp[i-2]
        if i >= 2 {
            let two_digit = (chars[i-2].to_digit(10).unwrap() * 10 + chars[i-1].to_digit(10).unwrap()) as i32;

            if two_digit >= 10 && two_digit <= 26 {
                dp[i] += dp[i-2]
            }
        }
    }
    dp[n]
}

fn main() {
    println!("111 -> {}", count_decodable("111"));   // expect 3
    println!("12 -> {}", count_decodable("12"));     // expect 2
    println!("27 -> {}", count_decodable("27"));     // expect 1
    println!("1111 -> {}", count_decodable("1111")); // expect 5
    println!("0 -> {}", count_decodable("0"));
    println!("-> {}", count_decodable(""));
}