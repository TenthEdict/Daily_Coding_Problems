// Daily Coding Problem: 77
// Difficulty: Easy
// Asked by: Snapchat
// Authored by: TenthEdict

fn merge_intervals(mut intervals: Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    if intervals.is_empty() {
        return vec![];
    }

    intervals.sort();

    let mut result = vec![intervals[0]];

    for interval in &intervals[1..] {
        let (a,b) = *result.last().unwrap();
        let (c,d) = *interval;

        if c <= b {
            result.pop();
            result.push((a, b.max(d)));
        } else {
            result.push(*interval);
        }
    }
    result
}

fn main() {
    println!("{:?}", merge_intervals(vec![(1, 3), (5, 8), (4, 10), (20, 25)]));
    println!("{:?}", merge_intervals(vec![]));
    println!("{:?}", merge_intervals(vec![(5, 10)]));
    println!("{:?}", merge_intervals(vec![(1, 10), (2, 5), (3, 7)]));
}

