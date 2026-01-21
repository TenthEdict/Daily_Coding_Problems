// Daily Coding Problem: 96
// Difficulty: Easy
// Asked by: Microsoft
// Author: TenthEdict

fn permute(current: &Vec<i32>, remaining: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    // base case
    if remaining.is_empty() {
        result.push(current.clone());
    }
    
    // recursive case
    for idx in 0..remaining.len() {
        let i = remaining[idx];
        let mut new_current = current.clone();
        new_current.push(i);
        let new_remaining = [&remaining[..idx], &remaining[idx+1..]].concat();
        result.extend(permute(&new_current, new_remaining));
    }
    result
}

fn main() {
    let case1: Vec<i32> = [1,2,3].to_vec();
    let case2: Vec<i32> = [1].to_vec();
    let case3: Vec<i32> = [1,2].to_vec();
    let case4: Vec<i32> = [1,2,3,4].to_vec();

    println!("{:?}", permute(&Vec::new(), case1));
    println!("{:?}", permute(&Vec::new(), case2));
    println!("{:?}", permute(&Vec::new(), case3));
    println!("{:?}", permute(&Vec::new(), case4));
}
