pub struct Solution {}

impl Solution {
    pub fn reverse_string(str: &mut Vec<char>) {
        if str.len() <= 1 {
            return;
        }

        let mut start = 0;
        let mut end = str.len() - 1;
        loop {
            let swap = str[start];
            str[start] = str[end];
            str[end] = swap;
            start += 1;
            end -= 1;
            if start >= end {
                break;
            }
        }
    }
}
