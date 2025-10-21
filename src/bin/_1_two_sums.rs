use std::collections::HashSet;
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }
        let set: HashSet<_> = nums.iter().cloned().collect();

        for (i, &v) in nums.iter().enumerate() {
            if set.contains(&(target - v)) {
                let complement = target - v;
                let rpos = nums.iter().rposition(|&x| x == complement).unwrap();
                if rpos > i {
                    return vec![i as i32, rpos as i32];
                }
            }
        }
        vec![]
    }
}

struct Solution;
fn main() {}

#[test]
fn test1() {
    let actual = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(actual, vec![0, 1]);
}
#[test]
fn test2() {
    let actual = Solution::two_sum(vec![3, 2, 4], 6);
    assert_eq!(actual, vec![1, 2]);
}