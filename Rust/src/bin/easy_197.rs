// Daily Coding Problem: 197
// Difficulty: Easy
// Asked by: Amazon
// Author: TenthEdict

fn rotate_right(arr: &mut [i32], k: usize) {
    if arr.is_empty() {
        return;
    }

    let k: usize = k % arr.len();

    if k == 0 { return; }

    arr.reverse();

    arr[0..k].reverse();

    arr[k..].reverse();

}

fn main() {
    let mut arr1 = vec![1, 2, 3, 4, 5];
    rotate_right(&mut arr1, 2);
    println!("{:?}", arr1);  // Expected: [4, 5, 1, 2, 3]

    let mut arr2 = vec![1, 2, 3];
    rotate_right(&mut arr2, 5);  // k > len case
    println!("{:?}", arr2);  // Expected: [2, 3, 1] (5 % 3 = 2)

    let mut arr3 = vec![1, 2, 3];
    rotate_right(&mut arr3, 3);  // k == len case
    println!("{:?}", arr3);  // Expected: [1, 2, 3] (no change)

    let mut arr4: Vec<i32> = vec![];
    rotate_right(&mut arr4, 2);  // empty array
    println!("{:?}", arr4);  // Expected: []
}