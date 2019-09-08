// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn append_pre(self, val: i32) -> Self {
        ListNode {
            val,
            next: Some(Box::new(self)),
        }
    }

    pub fn vals(mut self) -> Vec<i32> {
        let mut n = &mut self.next;
        let mut result = vec![self.val];
        loop {
            if n.is_none() {
                break;
            }
            result.push(n.as_mut().unwrap().val);
            n = &mut n.as_mut().unwrap().next;
        }
        result.reverse();
        result
    }

    fn vals_num(self) -> i128 {
        self.vals()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<i128>()
            .unwrap()
    }
}

pub struct Solution {}

impl Solution {
    pub fn vec_to_listnode(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result: ListNode = ListNode::new(0);
        for (index, item) in v.iter().enumerate() {
            result = if index == 0 {
                ListNode::new(*item)
            } else {
                result.append_pre(*item)
            }
        }
        Some(Box::new(result))
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_vals = l1.unwrap().vals_num();
        let l2_vals = l2.unwrap().vals_num();
        let sum = l1_vals + l2_vals;
        let mut sum = sum.to_string().chars().collect::<Vec<_>>();
        sum.reverse();
        let sum = sum.iter().map(|x| x.to_digit(10).unwrap() as i32).collect();
        Solution::vec_to_listnode(sum)
    }
}
