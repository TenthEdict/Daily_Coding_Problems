// Daily Coding Problem: 118
// Difficulty: Easy
// Asked by: Google
// Authored by: TenthEdict

fn sorted_squares(arr: &[i32]) -> Vec<i32>  {
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;
    let mut result: Vec<i32> = vec![0; arr.len()];
    let mut position: usize = arr.len() - 1;

    while left <= right {
        if arr[left].pow(2) > arr[right].pow(2) {
            result[position] = arr[left].pow(2);
            left += 1;
            if position > 0 {
                position -= 1;
            }
        } else {
            result[position] = arr[right].pow(2);
            right -= 1;
            if position > 0 {
                position -= 1;
            } 
        }
    }
    result
}
fn main() {
    let arr: Vec<i32> = vec![-9, -2, 0, 2, 3];
    let arr2: Vec<i32> = vec![-3, -1, 2, 4];
    println!("{:?}", sorted_squares(&arr2)); // Expected: [1, 4, 9, 16]
    println!("{:?}", sorted_squares(&arr)); // Expected: [0, 4, 4, 9, 81]
}
