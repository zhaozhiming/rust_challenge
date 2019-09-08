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
        let l1_vals = &mut l1.unwrap().vals();
        let l2_vals = &mut l2.unwrap().vals();
        let mut sum_list = vec![];
        // 进位：0 无需进位 1 需要进位
        let mut carry = 0;
        loop {
            // 取node最后一个数字，如果node为空则为0
            let n1 = if l1_vals.len() == 0 {
                0
            } else {
                l1_vals.pop().unwrap()
            };
            let n2 = if l2_vals.len() == 0 {
                0
            } else {
                l2_vals.pop().unwrap()
            };
            // 两数相加
            let mut sum = n1 + n2;
            // 如果有进位则加1，然后重置进位标志
            if carry == 1 {
                sum = sum + carry;
                carry = 0;
            }
            let result;
            // 如果相加结果大于等于10，则求余取余数并将进位标志为1
            if sum >= 10 {
                result = sum % 10;
                carry = 1;
            } else {
                result = sum;
            }
            sum_list.push(result);
            if l1_vals.len() == 0 && l2_vals.len() == 0 && carry == 0 {
                break;
            }
        }

        Solution::vec_to_listnode(sum_list)
    }
}
