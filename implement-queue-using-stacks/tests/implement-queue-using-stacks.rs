use implement_queue_using_stacks::MyQueue;

#[test]
fn implement_queue_using_stacks_new() {
    let my_queue = MyQueue::new();
    let expect: Vec<i32> = vec![];
    assert_eq!(expect, my_queue.queue);
}

#[test]
fn implement_queue_using_stacks_push() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1);
    assert_eq!(vec![1], my_queue.queue);
    my_queue.push(2);
    my_queue.push(3);
    assert_eq!(vec![3, 2, 1], my_queue.queue);
}

#[test]
fn implement_queue_using_stacks_pop() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1);
    my_queue.push(2);
    my_queue.push(3);
    assert_eq!(1, my_queue.pop());
    assert_eq!(vec![3, 2], my_queue.queue);
}

#[test]
fn implement_queue_using_stacks_peek() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1);
    my_queue.push(2);
    my_queue.push(3);
    assert_eq!(1, my_queue.peek());
    assert_eq!(vec![3, 2, 1], my_queue.queue);
}

#[test]
fn implement_queue_using_stacks_empty() {
    let mut my_queue = MyQueue::new();
    assert_eq!(true, my_queue.empty());
    my_queue.push(1);
    assert_eq!(false, my_queue.empty());
}
