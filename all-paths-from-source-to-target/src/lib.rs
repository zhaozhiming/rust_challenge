pub struct Solution {}

impl Solution {
    fn dfs(
        pos: usize,
        path: &mut Vec<i32>,
        visit: &mut Vec<bool>,
        graph: &Vec<Vec<i32>>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // 先在单次路径在添加该节点
        path.push(pos as i32);
        // 到最后一个节点时将单次路径添加到结果数组中
        if pos == graph.len() - 1 {
            let path_slice = &path[..];
            result.push(path_slice.to_vec());
        } else {
            // 遍历节点的其他路径
            for g in graph.get(pos).unwrap() {
                let next = *g as usize;
                if !visit[next] {
                    Solution::dfs(next, path, visit, graph, result);
                }
            }
        }

        // 如果没有匹配到则在单次路径中去除该节点
        path.pop();
        visit[pos] = false;
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 记录结果的数组
        let mut result = vec![];
        // 用来记录该节点是否已经访问过，初始值为未访问过
        let mut visit = vec![false; graph.len()];
        // 单次从开始到终点的路径
        let mut path = vec![];
        Solution::dfs(0, &mut path, &mut visit, &graph, &mut result);
        return result;
    }
}
