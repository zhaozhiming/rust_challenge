use reverse_string::Solution;

#[test]
fn test_reverse_string_sample() {
    let mut str = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut str);
    assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], str);
}

#[test]
fn test_reverse_string_normal() {
    let mut str = vec!['H','a','n','n','a','h'];
    Solution::reverse_string(&mut str);
    assert_eq!(vec!['h','a','n','n','a','H'], str);
}


#[test]
fn test_reverse_string_one() {
    let mut str = vec!['/'];
    Solution::reverse_string(&mut str);
    assert_eq!(vec!['/'], str);
}
