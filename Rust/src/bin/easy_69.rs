// Daily Coding Problem: 69
// Difficulty: Easy
// Asked by: Facebook
// Authored by: TenthEdict

fn largest_product_of_three(arr: &[i32]) -> Option<i32> {
    // 1. Length check
    if arr.len() < 3 {
        return None;
    }
    // 2. Initialize trackers (two smallest, three largest)
    let (mut small_1, mut small_2) =  (i32::MAX, i32::MAX);
    let (mut large_1, mut large_2, mut large_3) = (i32::MIN, i32::MIN, i32::MIN);
    // 3. Single pass through arr
    for num in arr {
        if *num > large_1 {
            large_3 = large_2;
            large_2 = large_1;
            large_1 = *num;
        } else if *num > large_2 {
            large_3 = large_2;
            large_2 = *num;
        } else if *num > large_3 {
            large_3 = *num;
        }

        if *num < small_1 {
            small_2 = small_1;
            small_1 = *num;
        } else if *num < small_2 {
            small_2 = *num;
        }
    }
    // 4. Compute both candidate products, return the max
    let two_smallest_product = small_1 * small_2 * large_1;
    let three_largest_product = large_1 * large_2 * large_3;

    if two_smallest_product > three_largest_product {
        return Some(two_smallest_product);
    } else {
        return Some(three_largest_product);
    }
}

fn main() {
    let numbers = [1,2,3,4,5,6];
    println!("{:?}", largest_product_of_three(&numbers));
    println!("{:?}", largest_product_of_three(&[-10, -10, 2, 3, 5])); // Expected: 500
    println!("{:?}", largest_product_of_three(&[1, 2, 3, 4, 5]));        // Expected: 60
    println!("{:?}", largest_product_of_three(&[-5, -4, -3, -2, -1]));   // Expected: -6
    println!("{:?}", largest_product_of_three(&[-100, 1, 2, 3]));        // Expected: 6
    println!("{:?}", largest_product_of_three(&[-5, -4, 0, 3, 2]));      // Expected: 60
    println!("{:?}", largest_product_of_three(&[1, 2]));                 // Expected: None
}
