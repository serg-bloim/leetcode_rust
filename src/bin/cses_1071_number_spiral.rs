#![allow(unused)]

use std::cmp::max;
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::marker::PhantomData;
use std::str::FromStr;

fn main() {
    let mut parser = Parser::new(stdin().lock());
    let tests: usize = parser.read_until_eol();
    for _ in 0..tests {
        let y: i64 = parser.read_until_whitespace();
        let x: i64 = parser.read_until_eol();
        println!("{}", calc_n(x - 1, y - 1));
    }
}

fn calc_n(x: i64, y: i64) -> i64 {
    let spiral_n = max(x, y);
    let spiral_dir = if spiral_n % 2 == 0 { 1 } else { -1 };
    let progress = spiral_n + (x - y) * spiral_dir;
    let n = 1 + spiral_n.pow(2) + progress;
    n
}

#[test]
fn test1(){
    assert_eq!(calc_n(2, 1), 8);
}
#[test]
fn test2(){
    assert_eq!(calc_n(0, 0), 1);
}
#[test]
fn test3(){
    assert_eq!(calc_n(1, 3), 15);
}

pub struct Parser<R: BufRead> {
    reader: R,
    buffer: Vec<u8>,
}
impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Parser<R> {
        Parser {
            reader,
            buffer: Vec::new(),
        }
    }
    fn read_until<T: FromStr>(&mut self, delim: u8) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        self.buffer.clear();
        self.reader.read_until(delim, &mut self.buffer);
        let str = std::str::from_utf8(&self.buffer).expect("err1");
        let n = str.trim().parse().expect("err2");
        n
    }
    fn read_until_eol<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        self.read_until(b'\n')
    }
    fn read_until_whitespace<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        self.read_until(b' ')
    }
    fn read_separated<'a, T: FromStr + 'a>(&'a mut self, delim: u8) -> impl Iterator<Item = T> + 'a
    where
        <T as FromStr>::Err: Debug,
    {
        struct SeparatedIterator<'a, T, R: BufRead> {
            parser: &'a mut Parser<R>,
            delim: u8,
            typearg: PhantomData<T>,
        }
        impl<'a, T: FromStr, R: BufRead> Iterator for SeparatedIterator<'a, T, R>
        where
            <T as FromStr>::Err: Debug,
        {
            type Item = T;
            fn next(&mut self) -> Option<T> {
                Some(self.parser.read_until(self.delim))
            }
        }
        SeparatedIterator {
            parser: self,
            delim,
            typearg: PhantomData,
        }
    }
}
