# 344. 反转字符串 

## leetcode 结果

* 执行用时: 24 ms, 在所有 Rust 提交中击败了 97.22% 的用户
* 内存消耗: 5.4 MB, 在所有 Rust 提交中击败了 5.26% 的用户

## 算法思路

#### 双指针

同时从数组的头和尾开始遍历，每次交换头和尾的元素，直到遍历到数组中间。

## 复杂度分析

* 时间复杂度：O(n)
* 空间复杂度：O(n)

## 困难与不足

暂无。

## 优化思路

内存消耗比较大，不知道有没有更好的方法减少内存消耗。

# 题型归类

双指针、字符串

## Leetcode 中相似题型有哪些

* [反转字符串中的元音字母](https://leetcode-cn.com/problems/reverse-vowels-of-a-string/)
* [反转字符串 II](https://leetcode-cn.com/problems/reverse-string-ii/)
