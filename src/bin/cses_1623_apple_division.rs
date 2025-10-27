
use std::cmp::min;
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    let mut apples = read_small_array::<i64>();
    let diff = find_min_diff(&mut apples);
    println!("{}", diff);
}
fn find_min_diff(apples: &mut Vec<i64>) -> i64 {
    apples.sort_by(|a, b| b.cmp(a));
    let apples = apples;
    let total: i64 = apples.iter().sum();
    let goal = total % 2;
    pick(total, &apples[..], goal)
}
fn pick(delta: i64, left: &[i64], goal: i64) -> i64 {
    if left.len() == 0 {
        return delta.abs();
    }
    if delta <= goal {
        return delta.abs();
    }

    let &next = left.first().unwrap();
    let next = next * 2;

    let delta_with_next = pick(delta - next, &left[1..], goal);
    if delta_with_next <= goal {
        return delta_with_next.abs();
    }
    let delta_without_next = pick(delta, &left[1..], goal);
    min(delta_with_next, delta_without_next)
}
fn read_small_array<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut stdin = stdin().lock();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("failed to read line");
    let _n: usize = line.trim().parse().expect("failed to parse N");
    line.clear();
    stdin.read_line(&mut line).expect("failed to read line");
    line.trim()
        .split_whitespace()
        .map(|x| x.parse().expect("failed to parse value"))
        .collect()
}

#[test]
fn test1() {
    let mut arr = vec![3, 2, 7, 4, 1];
    let diff = find_min_diff(&mut arr);
    assert_eq!(diff, 1);
}

#[test]
fn test2() {
    let mut arr = vec![952, 775, 292, 702, 859, 719, 65, 943, 376, 490];
    let diff = find_min_diff(&mut arr);
    assert_eq!(diff, 1);
}
#[test]
fn test3() {
    let mut arr = "13048212 423374770 19874608 812293014 860896267 224937483 907570920 952166494 450127366 887381168 966393898 102318919 723213664 491414754 571209206 99007249 302987622 263275846 36174214 727737543".split_whitespace().map(|x|x.parse().unwrap()).collect();
    let diff = find_min_diff(&mut arr);
    assert_eq!(diff, 8231);
}