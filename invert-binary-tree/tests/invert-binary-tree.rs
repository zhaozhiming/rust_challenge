use invert_binary_tree::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn build_binary_tree(v: Vec<i32>) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new(v[0])));
    let left = Rc::new(RefCell::new(TreeNode::new(v[1])));
    let right = Rc::new(RefCell::new(TreeNode::new(v[2])));
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());

    let left_left = Rc::new(RefCell::new(TreeNode::new(v[3])));
    let left_right = Rc::new(RefCell::new(TreeNode::new(v[4])));
    left.borrow_mut().left = Some(left_left.clone());
    left.borrow_mut().right = Some(left_right.clone());

    let right_left = Rc::new(RefCell::new(TreeNode::new(v[5])));
    let right_right = Rc::new(RefCell::new(TreeNode::new(v[6])));
    right.borrow_mut().left = Some(right_left.clone());
    right.borrow_mut().right = Some(right_right.clone());

    return root;
}

#[test]
fn test_invert_tree_sample() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    let result = Solution::invert_tree(Some(root));
    assert_eq!(Some(right.clone()), result.unwrap().borrow_mut().left);
}

#[test]
fn test_invert_tree_normal() {
    let tree = build_binary_tree(vec![4, 2, 7, 1, 3, 6, 9]);
    let expect = build_binary_tree(vec![4, 7, 2, 9, 6, 3, 1]);
    let result = Solution::invert_tree(Some(tree));
    assert_eq!(Some(expect), result);
}
