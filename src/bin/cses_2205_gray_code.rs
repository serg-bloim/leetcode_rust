use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    let n = read_single();
    let range = (1..=n).rev();
    for l in 0..2u32.pow(n) {
        for n in range.clone() {
            let shift = 2u32.pow(n) / 2;
            // let shift = 0;
            print!("{}", ((l + shift) / (2u32.pow(n))) % 2)
        }
        println!()
    }
}

#[test]
fn test1() {
    for l in 0..32 {
        for n in 1..8 {
            let shift = 2u32.pow(n) / 2;
            // let shift = 0;
            print!("{}", ((l + shift) / (2u32.pow(n))) % 2)
        }
        println!()
    }
}
#[test]
fn test4() {}

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
