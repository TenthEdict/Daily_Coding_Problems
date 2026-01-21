// Daily Coding Problem: 282
// Difficulty: Easy
// Asked by: Netflix
// Authored by: TenthEdict

// Question: Given an array of integers, determine whether it contains a Pythagorean triplet.
// Recall that a Pythogorean triplet (a, b, c) is defined by the equation a2+ b2= c 2.

use std::collections::HashSet;

fn contains_pythagorean_triplet(arr: &Vec<i32>) -> bool {
    let squared: HashSet<i32> = arr.iter().map(|x| x.pow(2)).collect();  

    for i in &squared {
        for j in &squared {
            if squared.contains(&(i+j)) {
                return true
            }
        }
    }
    return false
}

fn main() {
    println!("{}", contains_pythagorean_triplet(&vec![1, 2, 3]));    // false
    println!("{}", contains_pythagorean_triplet(&vec![5, 12, 13]));  // true
    println!("{}", contains_pythagorean_triplet(&vec![]));           // false
}
