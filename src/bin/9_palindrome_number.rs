impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false }
        let str = x.to_string();
        let chars = str.chars();
        chars.zip(str.chars().rev()).all(|(a, b)| a == b)
    }
}
struct Solution;
fn main() {}

#[test]
fn test1() {
    let actual = Solution::is_palindrome(121);
    assert_eq!(actual, true);
}
#[test]
fn test2() {
    let actual = Solution::is_palindrome(-121);
    assert_eq!(actual, false);
}
#[test]
fn test3() {
    let actual = Solution::is_palindrome(10);
    assert_eq!(actual, false);
}