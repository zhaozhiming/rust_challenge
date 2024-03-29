# 两数相加

## leetcode 结果

- 执行用时 : 4 ms, 在所有 Rust 提交中击败了 92.97% 的用户
- 内存消耗 : 2.1 MB, 在所有 Rust 提交中击败了 100.00% 的用户

## 算法思路

#### 数字转换

这种思路是把 2 个 ListNode 转换成数字，然后把 2 个数字相加，最后再把结果转换成 ListNode。

这种方式只能针对覆盖一部分 leetcode 的测试案例，对于超过`i128`位的 ListNode 来说就会报错。

#### 转换数组

将 2 个 ListNode 转换成数组，然后将 2 个数组的每个元素进行相加,最后将结果转换成 ListNode。

## 困难与不足

- 在 leetcode 上编码时踩坑，发现不能改给定的`struct`
- 对 Rust 的借用和所有权还不熟，在代码调试时遇到该类问题要被编译器虐半天

## 优化思路

~~将 2 个 ListNode 转成数组，然后再遍历相加，现在在尝试这种方式，但是还没做完。~~

已经使用该算法完成题目。

# 题型归类

链表、数学

## Leetcode 中相似题型有哪些

- [字符串相乘](https://leetcode-cn.com/problems/multiply-strings/)
- [二进制求和](https://leetcode-cn.com/problems/add-binary/)
- [两整数之和](https://leetcode-cn.com/problems/sum-of-two-integers/)
