# 两数之和

## leetcode 结果

| 题目 | 状态 | 执行用时 | 语言 |
| --- | --- | --- | --- |
| 两数之和 | 通过	| 64 ms	| rust |

## 算法思路

暴力破解，用到了 2 层循环，第一层循环先遍历数组的每个元素，然后再遍历数组的剩下元素，如果两数相加等于目标结果则返回这 2 个元素的数组下标，否则继续循环。

## 困难与不足

困难的地方是 Rust 的语法不熟，很多 API 要查文档

## 优化思路

没有想到更好的方式，求指教

# 题型归类

数组、哈希表

## Leetcode 中相似题型有哪些

* [三数之和](https://leetcode-cn.com/problems/3sum/)
* [四数之和](https://leetcode-cn.com/problems/4sum/)
* [两数之和 II - 输入有序数组](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)
