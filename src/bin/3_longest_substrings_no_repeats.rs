impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_distr = [0usize; 255];
        let mut start = 0;
        let mut max_size = 0;
        for (i, c) in s.chars().enumerate() {
            let ccode = c as usize;
            if ccode >= char_distr.len() {
                panic!("wrong char")
            }
            if char_distr[ccode] >= 1 {
                let size = i - start;
                if size > max_size {
                    max_size = size;
                }
                for fc in s[start..i].chars() {
                    let ccode = fc as usize;
                    char_distr[ccode] -= 1;
                    start += 1;
                    if fc == c {break}
                }
            }
            char_distr[ccode] += 1;
        }
        let last_size = s.len() - start;
        if last_size > max_size {max_size = last_size;}
        max_size as i32
    }
}
struct Solution;
fn main() {}

#[test]
fn test1() {
    let str = "abcabcbb";
    let actual = Solution::length_of_longest_substring(String::from(str));
    assert_eq!(actual, 3);
}
#[test]
fn test2() {
    let str = "bbbbb";
    let actual = Solution::length_of_longest_substring(String::from(str));
    assert_eq!(actual, 1);
}
#[test]
fn test3() {
    let str = "pwwkew";
    let actual = Solution::length_of_longest_substring(String::from(str));
    assert_eq!(actual, 3);
}

#[test]
fn test4() {
    let str = " ";
    let actual = Solution::length_of_longest_substring(String::from(str));
    assert_eq!(actual, 1);
}

#[test]
fn test5() {
    let str = "abcdef";
    let actual = Solution::length_of_longest_substring(String::from(str));
    assert_eq!(actual, 6);
}
