#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MyQueue {
    pub queue: Vec<i32>,
    transfer: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            queue: Vec::new(),
            transfer: Vec::new(),
        }
    }

    pub fn ship(source: &mut Vec<i32>, target: &mut Vec<i32>) {
        while source.len() > 0 {
            target.push(source.pop().unwrap());
        }
    }

    pub fn push(&mut self, x: i32) {
        // 先把栈数据转移到中转栈
        self.transfer = Vec::new();
        MyQueue::ship(&mut self.queue, &mut self.transfer);
        // 相当在空栈里面放数据
        self.queue.push(x);
        // 最后把中转栈的数据又放回到栈中
        MyQueue::ship(&mut self.transfer, &mut self.queue);
    }

    pub fn pop(&mut self) -> i32 {
        if self.queue.len() > 0 {
            return self.queue.pop().unwrap();
        }
        panic!("Queue is empty.");
    }

    pub fn peek(&mut self) -> i32 {
        let result = self.queue.pop().unwrap();
        self.queue.push(result);
        result
    }

    pub fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}
