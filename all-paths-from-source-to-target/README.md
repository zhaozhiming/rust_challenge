# 797. 所有可能的路径

## leetcode 结果

* 执行用时: 12 ms, 在所有 Rust 提交中击败了 100.00% 的用户
* 内存消耗: 2.4 MB, 在所有 Rust 提交中击败了 100.00% 的用户

## 算法思路

#### 深度优先搜索

从第一个节点开始，采用深度优先搜索的方式进行遍历，记录访问节点的路径，直到倒数第二个节点时将本次访问路径记录到结果中。

## 复杂度分析

* 时间复杂度：O(V+E)
* 空间复杂度：O(V+E)

## 困难与不足

开始连题目都看不懂，后面在群里和大家讨论了之后才理解了题目的意思，其实只要把有向图画出来就比较好理解了。

## 优化思路

暂无

# 题型归类

深度优先搜索、递归

## Leetcode 中相似题型有哪些

* [验证二叉搜索树](https://leetcode-cn.com/problems/validate-binary-search-tree/)
* [对称二叉树](https://leetcode-cn.com/problems/symmetric-tree/)
