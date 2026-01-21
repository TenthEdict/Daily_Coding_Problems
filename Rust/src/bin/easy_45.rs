// Daily Coding Problem: 45
// Difficulty: Easy
// Asked by: Two Sigma
// Author: TenthEdict

use rand::Rng;

fn rand5() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=5)  // 1 to 5 inclusive
}

fn rand7() -> i32 {
    let row = rand5();
    let col = rand5();
    let cell = (row - 1) * 5 + col;

    if cell <= 21 {
        return (cell - 1) % 7 + 1;
    } else {
        rand7()
    }
}

fn main() {
    let mut counts = [0; 7];  // indices 0-6 for values 1-7
    
    for _ in 0..10000 {
        let result = rand7();
        counts[(result - 1) as usize] += 1;
    }
    
    for i in 1..=7 {
        println!("{}: {}", i, counts[i - 1]);
    }
}