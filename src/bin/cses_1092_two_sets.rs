#![allow(unused)]

use std::cmp::max;
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::iter::{Chain, once};
use std::marker::PhantomData;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let n: i32 = read_single();
    if n == 3{
        println!("YES");
        println!("2");
        println!("1 2");
        println!("1");
        println!("3");
    } else if n % 4 == 0 {
        println!("YES");
        println!("{}", n / 2);
        print_iter((1..=n / 4).chain(3 * n / 4 + 1..=n));
        println!();
        println!("{}", n / 2);
        print_iter(n / 4 + 1..=3 * n / 4);
    } else if (n + 1) % 4 == 0 {
        let t = n + 1;
        let middle = (t) / 2;
        println!("YES");
        println!("{}", t / 2);
        print_iter((1..=t / 4).chain(3 * t / 4 + 1..t).chain(once(middle)));
        println!();
        println!("{}", t / 2 - 1);
        print_iter((t / 4 + 1..=3 * t / 4).filter(|&x| x != middle));
    }else {
        println!("NO");
    }
}
fn print_iter(mut range: impl Iterator<Item = i32>) {
    print!("{}", range.next().unwrap());
    range.for_each(|x| print!(" {}", x));
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
