// Daily Coding Problem: 225
// Difficulty: Easy
// Asked by: Bloomberg
// Authored by: TenthEdict

fn josephus_on2(n: usize, k: usize) -> usize {
    let mut prisoners: Vec<usize> = (1..=n).collect();
    let mut index: usize = k - 1;
    while prisoners.len() > 1 {
        prisoners.remove(index);
        index = (index + k - 1) % prisoners.len();
    }
    prisoners[0]
}

fn josephus_ologn(n: usize) -> usize {
    let mut p: usize = 1;
    while p * 2 <= n {
        p = p * 2;
    }
    2 * (n - p) + 1
}

fn main() {
    println!("Comparing O(n²) and O(log n) solutions for k=2:");
    for n in 1..=16 {
        let slow = josephus_on2(n, 2);
        let fast = josephus_ologn(n);
        let check = if slow == fast { "✓" } else { "✗" };
        println!("n={}: O(n²)={}, O(log n)={} {}", n, slow, fast, check);
    }
}