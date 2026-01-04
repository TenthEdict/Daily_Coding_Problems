// Daily Coding Problem: 43
// Difficulty: Easy
// Asked by: Amazon
// Authored by: TenthEdict

struct MaxStack {
    pushed_vals: Vec<i32>,
    running_max: Vec<i32>
}

impl MaxStack {
    fn new() -> Self {
        Self {
            pushed_vals: Vec::new(),
            running_max: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.pushed_vals.push(val);
        let new_max = match self.running_max.last() {
            Some(current_max) => if val > *current_max { val } else { *current_max },
            None => val,
        };
        self.running_max.push(new_max);
    }
    
    fn pop(&mut self) -> Option<i32> {
        self.running_max.pop();
        self.pushed_vals.pop()
    }
    
    fn max(&self) -> Option<i32> {
        self.running_max.last().copied()
    }
}

fn main() {
    let mut arr = MaxStack::new();

    arr.push(3);
    arr.push(1);
    arr.push(5);
    arr.push(2);

    println!("{:?}", arr.max());

    arr.pop();

    println!("{:?}", arr.max());

    arr.pop();

    println!("{:?}", arr.max());

    arr.pop();
    arr.pop();
    println!("{:?}", arr.max()); 
}