// Daily Coding Problem: 47
// Difficulty: Easy
// Asked by: Amazon
// Author: TenthEdict

fn best_profit(arr: &[i32]) -> Option<i32> {
    if arr.len() < 2 {
        return Some(0);
    }

    let mut min_price: i32 = arr[0];
    let mut profit: i32 = 0;

    for price in arr {
        if *price < min_price {
            min_price = *price;
        } 
        profit = profit.max(*price - min_price);
    }
    Some(profit)
}

fn main() {
    let cases: Vec<(&[i32], i32)> = vec![
        (&[9, 11, 8, 5, 7, 10], 5),
        (&[10, 2, 11, 1, 5], 9),
        (&[10, 8, 5, 2], 0),
        (&[5], 0),
        (&[], 0),
    ];

    for (arr, expected) in cases {
        let result = best_profit(arr);
        let pass = if result == Some(expected) { "✓" } else { "✗" };
        println!("{:?} -> {:?} (expected {}) {}", arr, result, expected, pass);
    }
}
