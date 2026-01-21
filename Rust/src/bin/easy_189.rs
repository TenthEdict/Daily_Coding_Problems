// Daily Coding Problem: 189
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

use std::collections::HashMap;

fn longest_distinct_subarray(arr: &[i32]) -> usize {
    let mut positions: HashMap<i32, usize> = HashMap::new();
    let mut beginning: usize = 0;
    let mut result: usize = 0;

    for (i, e) in arr.iter().enumerate() {
        if let Some(old_pos) = positions.insert(*e, i) {
            if old_pos >= beginning {
                beginning = old_pos + 1;
            }
        }
        result = result.max(i - beginning + 1);
    }
    result
}

fn main() {
    println!("{}", longest_distinct_subarray(&[1, 2, 3, 2, 4, 5])); // 4
    println!("{}", longest_distinct_subarray(&[5, 1, 3, 5, 2, 3, 4, 1])); // 5
    println!("{}", longest_distinct_subarray(&[1, 1, 1, 1])); // 1
    println!("{}", longest_distinct_subarray(&[1, 2, 3, 4, 5])); // 5
    println!("{}", longest_distinct_subarray(&[])); // 0
}
