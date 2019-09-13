#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MyQueue {
    pub queue: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue { queue: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        let first = *self.queue.first().unwrap();
        self.queue.remove(0);
        first
    }

    pub fn peek(&self) -> i32 {
        *self.queue.first().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}
