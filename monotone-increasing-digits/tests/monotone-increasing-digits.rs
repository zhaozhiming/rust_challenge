use monotone_increasing_digits;

#[test]
fn test_monotone_increasing_digits() {
    assert_eq!(9, monotone_increasing_digits::monotone_increasing_digits(10));
    assert_eq!(299, monotone_increasing_digits::monotone_increasing_digits(332));
    assert_eq!(1234, monotone_increasing_digits::monotone_increasing_digits(1234));
    assert_eq!(0, monotone_increasing_digits::monotone_increasing_digits(0));
    assert_eq!(667999, monotone_increasing_digits::monotone_increasing_digits(668841));
    assert_eq!(99999999, monotone_increasing_digits::monotone_increasing_digits(101695217));
    assert_eq!(399999999, monotone_increasing_digits::monotone_increasing_digits(401695217));
}
