use add_two_numbers::Solution;

#[test]
fn test_add_two_numbers_1() {
    let l1 = Solution::vec_to_listnode(vec![2, 4, 5]);
    let l2 = Solution::vec_to_listnode(vec![5, 6, 4]);
    let expect = Solution::vec_to_listnode(vec![9, 0, 8]);
    assert_eq!(expect, Solution::add_two_numbers(l1, l2));
}

#[test]
fn test_add_two_numbers_2() {
    let l1 = Solution::vec_to_listnode(vec![9]);
    let l2 = Solution::vec_to_listnode(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 1]);
    let expect = Solution::vec_to_listnode(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(expect, Solution::add_two_numbers(l1, l2));
}
