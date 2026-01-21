// Daily Coding Problem: 37
// Difficulty: Easy
// Asked by: Google
// Author: TenthEdict

fn power_set(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let n: usize = arr.len();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..(1 << n) {
        let mut subset: Vec<i32> = Vec::new();

        for j in 0..n {
            if i & (1 << j) != 0 {
                subset.push(arr[j]);
            }
        }
        result.push(subset);
    }
    result
}

fn main() {
    let set1 = vec![1, 2, 3];
    println!("{:?}", power_set(set1));
    
    let set2 = vec![1, 2];
    println!("{:?}", power_set(set2));
    
    let set3: Vec<i32> = vec![];
    println!("{:?}", power_set(set3));
}
