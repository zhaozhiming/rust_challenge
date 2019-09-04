use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 用hashmap来保存遍历结果，key为数组数字，value为该数字索引
    let mut map = HashMap::new();
    for (i, item) in nums.iter().enumerate() {
        // 组成目标的另外一个数字
        let rest = target - item;
        let pre = map.get(&rest);
        // 如果hashmap没有这个数字则保存，有的话则返回结果
        if pre.is_some() {
            // 之前保存的索引
            let pre_value = pre.unwrap();
            return vec![*pre_value as i32, i as i32];
        } else {
            map.insert(item, i);
        }
    }

    panic!("No two sum solution")
}

pub fn two_sum_violence(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
