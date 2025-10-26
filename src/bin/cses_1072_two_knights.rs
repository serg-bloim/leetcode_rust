#![allow(unused)]

use std::cmp::max;
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::marker::PhantomData;
use std::str::FromStr;

fn main() {
    let mut parser = Parser::new(stdin().lock());
    let n: usize = parser.read_until_eol();
    for k in 1..=n {
        println!("{}", calc_knight_combos(k as i64))
    }
}
/**
|-------------
|c1|sc|s1|s1|
|-------------
|sc|c2|s2|s2|
|-------------
|s1|s2|cr|cr|

c1 - corner outer, 2 attack spots
c2 - corner inner, 4 attack spots
sc - side corner, 3 attack spots
s1 - side outer, 4 attack spots
s2 - side inner, 6 attack spots
cr - center, 8 attack spots

**/
fn calc_knight_combos(k: i64) -> i64 {
    const ATTACK_POSITIONS_CENTER: i64 = 8;
    const ATTACK_POSITIONS_SIDE_OUTER: i64 = 4;
    const ATTACK_POSITIONS_SIDE_INNER: i64 = 6;
    const ATTACK_POSITIONS_CORNER_OUTER: i64 = 2;
    const ATTACK_POSITIONS_CORNER_INNER: i64 = 4;
    const ATTACK_POSITIONS_SIDE_CORNER: i64 = 3;

    match k {
        1 => 0,
        2 => 6,
        3 => 28,
        _ => {
            let corners = 4;
            let side_corners = corners * 2;
            let sides = (k - 4) * 4;
            let centers = max(k - 4, 0).pow(2);
            let kp2 = k.pow(2) - 1;
            let n = centers * (kp2 - ATTACK_POSITIONS_CENTER)
                + corners * (kp2 - ATTACK_POSITIONS_CORNER_OUTER)
                + corners * (kp2 - ATTACK_POSITIONS_CORNER_INNER)
                + side_corners * (kp2 - ATTACK_POSITIONS_SIDE_CORNER)
                + sides * (kp2 - ATTACK_POSITIONS_SIDE_OUTER)
                + sides * (kp2 - ATTACK_POSITIONS_SIDE_INNER);
            n / 2
        }
    }
}

#[test]
fn test1() {
    assert_eq!(calc_knight_combos(2), 6);
}
#[test]
fn test2() {
    assert_eq!(calc_knight_combos(3), 28);
}
#[test]
fn test3() {
    assert_eq!(calc_knight_combos(4), 96);
}
#[test]
fn test4() {
    assert_eq!(calc_knight_combos(5), 252);
}
#[test]
fn test_5() {
    assert_eq!(calc_knight_combos(10_000), 252);
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
