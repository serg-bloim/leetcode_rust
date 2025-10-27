#![allow(unused)]

use std::cmp::max;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Lines, StdinLock, stdin};
use std::iter::Map;
use std::marker::PhantomData;
use std::str::FromStr;

fn main() {
    for test in read_tests::<u32>() {
        if let &[a, b] = &test[..] {
            if find_coefs(a, b).is_some() {
                println!("YES")
            } else {
                println!("NO")
            }
        }
    }
}

fn find_coefs(x: u32, y: u32) -> Option<(u32, u32)> {
    if x > 2 * y {
        return None;
    }
    if y == 0 && x != 0 {
        return None;
    }
    if x < y {
        return find_coefs(y, x);
    }
    let t = (2 * x - y) / 3;
    let t_rem = (2 * x - y) % 3;
    if t_rem == 0 {
        let k = x - 2 * t;
        Some((t, k))
    } else {
        None
    }
}

#[test]
fn test1() {
    assert!(find_coefs(4, 5).is_some());
    assert!(find_coefs(6, 6).is_some());
    assert!(find_coefs(5, 10).is_some());
}
#[test]
fn test4() {
    assert!(find_coefs(5, 5).is_none());
    assert!(find_coefs(2, 2).is_none());
    assert!(find_coefs(5, 11).is_none());
    assert!(find_coefs(3, 12).is_none());
}
fn read_tests<T>() -> impl Iterator<Item = Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut lines = stdin().lock().lines();
    let n = lines.next().unwrap().unwrap().parse().unwrap();
    lines
        .map(|line| {
            line.expect("read error")
                .split_whitespace()
                .map(|value| value.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .take(n)
}
