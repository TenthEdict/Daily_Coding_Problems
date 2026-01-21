// Daily Coding Problem: 21
// Difficulty: Easy
// Asked by: Snapchat
// Author: TenthEdict

fn minimum_rooms(arr: &Vec<(i32,i32)>) -> i32 {
    let mut events: Vec<(i32, i32)> = Vec::new();
    let mut current_rooms: i32 = 0;
    let mut max_rooms: i32 = 0;

    for interval in arr {
        events.push((interval.0, 1));
        events.push((interval.1, -1));
    }

    events.sort();

    for event in &events {
        current_rooms += event.1;
        max_rooms = max_rooms.max(current_rooms);
    }
    max_rooms
}

fn main() {
    // Example from problem: should return 2
    let intervals = vec![(30, 75), (0, 50), (60, 150)];
    println!("{:?} -> {}", intervals, minimum_rooms(&intervals));
    
    // Edge case: no overlap, should return 1
    let intervals2 = vec![(0, 10), (20, 30), (40, 50)];
    println!("{:?} -> {}", intervals2, minimum_rooms(&intervals2));
    
    // Edge case: all overlap, should return 3
    let intervals3 = vec![(0, 100), (0, 100), (0, 100)];
    println!("{:?} -> {}", intervals3, minimum_rooms(&intervals3));
}
