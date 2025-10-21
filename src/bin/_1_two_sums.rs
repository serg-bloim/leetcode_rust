impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }
        let candidate = nums.first().unwrap();
        let rest = &nums[1..];
        return Self::find_num(target, candidate, rest, 0);
    }
    fn find_num(target: i32, candidate: &i32, rest: &[i32], offset: i32) -> Vec<i32> {
        if rest.is_empty() {
            return vec![];
        }
        let goal = target - candidate;
        if let Some(index) = rest.iter().position(|&x| x == goal) {
            return vec![offset, 1 + offset + index as i32];
        }
        Self::find_num(target, rest.first().unwrap(), &rest[1..], offset + 1)
    }
}



struct Solution;
fn main() {}

#[test]
fn test1(){
    let actual = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(actual, vec![0, 1]);
}
#[test]
fn test2(){
    let actual = Solution::two_sum(vec![3, 2, 4], 6);
    assert_eq!(actual, vec![1, 2]);
}