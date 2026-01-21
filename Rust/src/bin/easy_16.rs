// Daily Coding Problem: 16
// Difficulty: Easy
// Asked by: Twitter
// Author: TenthEdict

struct Buffer {
    buffer: Vec<i32>,
    capacity: usize,
    write_index: usize
}
impl Buffer {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            capacity,
            write_index: 0
        }
    }
    
    fn record(&mut self, order_id: i32) {
        // Add order_id at current index, then advance index (with wrap-around)
        self.buffer[self.write_index] = order_id;
        self.write_index = (self.write_index + 1) % self.capacity;
    }
    
    fn get_last(&self, i: usize) -> i32 {
        // Calculate the index of the ith last element
        let index = (self.write_index + self.capacity - i) % self.capacity;
        self.buffer[index]
    }
}

fn main() {
    let mut log = Buffer::new(4);
    
    log.record(100);
    log.record(200);
    log.record(300);
    log.record(400);
    log.record(500);  // This overwrites 100
    
    println!("Last 1: {}", log.get_last(1));  // Should print 500
    println!("Last 2: {}", log.get_last(2));  // Should print 400
    println!("Last 4: {}", log.get_last(4));  // Should print 200
}
