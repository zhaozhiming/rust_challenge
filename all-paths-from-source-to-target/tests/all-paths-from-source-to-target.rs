use all_paths_from_source_to_target::Solution;

#[test]
fn test_all_paths_source_target_sample() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let expect = vec![vec![0, 1, 3], vec![0, 2, 3]];
    assert_eq!(expect, Solution::all_paths_source_target(graph));
}

#[test]
fn test_all_paths_source_target_normal() {
    let graph = vec![
        vec![2, 4],
        vec![5],
        vec![1, 3],
        vec![1, 4, 5],
        vec![5],
        vec![],
    ];
    let expect = vec![
        vec![0, 2, 1, 5],
        vec![0, 2, 3, 1, 5],
        vec![0, 2, 3, 4, 5],
        vec![0, 2, 3, 5],
        vec![0, 4, 5],
    ];
    assert_eq!(expect, Solution::all_paths_source_target(graph));
}
