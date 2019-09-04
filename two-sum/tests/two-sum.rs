use two_sum;

#[test]
fn test_two_sum_hashmap() {
    assert_eq!(vec![0, 1], two_sum::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 3], two_sum::two_sum(vec![40, 50, 60, 70], 120));
    assert_eq!(vec![1, 2], two_sum::two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 2], two_sum::two_sum(vec![3, 2, 3], 6));
    assert_eq!(vec![0, 1], two_sum::two_sum(vec![3, 3, 2], 6));
}

#[test]
fn test_two_sum_violence() {
    assert_eq!(vec![0, 1], two_sum::two_sum_violence(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 3], two_sum::two_sum_violence(vec![40, 50, 60, 70], 120));
    assert_eq!(vec![1, 2], two_sum::two_sum_violence(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 2], two_sum::two_sum_violence(vec![3, 2, 3], 6));
    assert_eq!(vec![0, 1], two_sum::two_sum_violence(vec![3, 3, 2], 6));
}
