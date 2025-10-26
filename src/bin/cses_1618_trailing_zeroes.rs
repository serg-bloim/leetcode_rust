#![allow(unused)]

use std::cmp::max;
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::iter::{Chain, once};
use std::marker::PhantomData;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let mut n: u32 = read_single();
    let zeroes = calc_zeroes(n);
    println!("{}", zeroes);
}

fn calc_zeroes(n: u32) -> u32 {
    let mut n = n;
    let mut zeroes = 0;
    while n > 0 {
        n = n / 5;
        zeroes += n;
    }
    zeroes
}
#[test]
fn test1() {
    assert_eq!(calc_zeroes(20), 4);
}

#[test]
fn test2() {
    assert_eq!(calc_zeroes(19), 3);
}
#[test]
fn test3() {
    assert_eq!(calc_zeroes(21), 4);
}
#[test]
fn test4() {
    assert_eq!(calc_zeroes(25), 6);
}

fn read_single<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    stdin()
        .lock()
        .lines()
        .next()
        .expect("No input provided")
        .expect("No input provided")
        .parse()
        .expect("Should be a number!")
}
