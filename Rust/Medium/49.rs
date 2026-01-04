// Daily Coding Problem: 49
// Difficulty: Medium
// Asked by: Amazon
// Authored by: TenthEdict

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut current_sum: i32 = 0;
    let mut max_sum: i32 = 0;

    for i in arr {
        current_sum += i;
        max_sum = max_sum.max(current_sum);
        if current_sum < 0 {
            current_sum = 0;
        }
    }
    max_sum
}

fn max_subarray_sum_nonempty(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    } else {
        let mut current_sum: i32 = 0;
        let mut max_sum: i32 = arr[0];

        for i in arr {
            current_sum += i;
            max_sum = max_sum.max(current_sum);
            if current_sum < 0 {
                current_sum = 0;
            }
        }
        return Some(max_sum)
    }
}

fn main() {
    println!("{:?}", max_subarray_sum_nonempty(&[34, -50, 42, 14, -5, 86])); // 137
    println!("{:?}", max_subarray_sum_nonempty(&[-5, -1, -8, -9]));          // 0
    println!("{:?}", max_subarray_sum_nonempty(&[1, 2, 3, 4]));              // 10
    println!("{:?}", max_subarray_sum_nonempty(&[]));                        // None
    println!("{:?}", max_subarray_sum_nonempty(&[-2, 1, -1, 2, -1, 2]));     // 3
}
