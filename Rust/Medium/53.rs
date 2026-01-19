// Daily Coding Problem: 53
// Difficulty: Medium
// Asked by: Apple
// Author: TenthEdict

struct Queue{
    inbox: Vec<i32>,
    outbox: Vec<i32>
}

impl Queue {
    fn new() -> Self {
        Self {
            inbox: Vec::new(),
            outbox: Vec::new()
        }
    }

    fn enqueue(&mut self, val: i32) {
        self.inbox.push(val)
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.outbox.is_empty() {
            while !self.inbox.is_empty() {
                self.outbox.push(self.inbox.pop().unwrap())
            }
        }
        self.outbox.pop() 
    }
}

fn main() {
    let mut q = Queue::new();
    
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    
    println!("{:?}", q.dequeue());  // Some(1)
    println!("{:?}", q.dequeue());  // Some(2)
    
    q.enqueue(4);
    q.enqueue(5);
    
    println!("{:?}", q.dequeue());  // Some(3)
    println!("{:?}", q.dequeue());  // Some(4)
    println!("{:?}", q.dequeue());  // Some(5)
    println!("{:?}", q.dequeue());  // None
}