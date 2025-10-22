impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let first = strs.first().unwrap();
        if strs.len() == 1 {
            return first.clone();
        }
        let prefix_length = strs[1..]
            .iter()
            .map(|x| common_prefix_length(x, first))
            .min()
            .unwrap_or(0);
        first[0..prefix_length].to_string()
    }
}

fn common_prefix_length(str1: &String, str2: &String) -> usize {
    str1.chars()
        .zip(str2.chars())
        .take_while(|(a, b)| a == b)
        .count()
}

struct Solution;
fn main() {}

#[test]
fn test1() {
    let strs = vec!["flower", "flow", "flight"];
    let vec1 = strs.into_iter().map(String::from).collect();
    let actual = Solution::longest_common_prefix(vec1);
    assert_eq!(actual, "fl");
}
#[test]
fn test2() {
    let strs = vec!["dog", "racecar", "car"];
    let vec1 = strs.into_iter().map(String::from).collect();
    let actual = Solution::longest_common_prefix(vec1);
    assert_eq!(actual, "");
}
