// Daily Coding Problem: 184
// Difficulty: Easy
// Asked by: Amazon
// Authored by: TenthEdict

fn gcd(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}

fn gcd_of_array(arr: &[i32]) -> i32 {
    let mut result = arr[0];
    for i in 1..arr.len() {
        result = gcd(result, arr[i])
    }
    result
}

fn main() {
    let numbers = [42, 56, 14];
    println!("{}", gcd_of_array(&numbers)); // returns 14 !
}
