// Daily Coding Problem: 33
// Difficulty: Easy
// Asked by: Microsoft
// Authored by: TenthEdict

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct RunningMedian {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>
}

impl RunningMedian {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add(&mut self, num: i32) {
        if self.max_heap.peek().map_or(true, |&top| num <= top) {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        if self.max_heap.len() > self.min_heap.len() + 1 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        }
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            return *self.max_heap.peek().unwrap() as f64;
        } else {
            return (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0;
        }
    }
}

fn main() {
    let mut median_arr = RunningMedian::new();
    let arr = [2, 1, 5, 6, 2, 0, 5];

    for i in arr {
        median_arr.add(i);
        println!("{}", median_arr.median())
    }
}