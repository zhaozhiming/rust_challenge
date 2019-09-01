pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, itemi) in nums[..nums.len() - 1].iter().enumerate() {
        let p = nums.iter().position(|x| x == itemi).unwrap() + 1;
        for (j, itemj) in nums[p..].iter().enumerate() {
            if itemi + itemj == target {
                return vec![i as i32, (j + p) as i32];
            }
        }
    }
    panic!("No two sum solution")
}
