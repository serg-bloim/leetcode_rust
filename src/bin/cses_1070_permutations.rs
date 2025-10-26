#![allow(unused)]
use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::marker::PhantomData;
use std::str::FromStr;

fn main() {
    let mut parser = Parser::new(stdin().lock());
    let n: usize = parser.read_until_eol();
    if n == 1 {
        print!("1")
    } else if n < 4 {
        print!("NO SOLUTION")
    } else {
        for a in (2..=n).step_by(2) {
            print!("{} ", a)
        }
        for a in (1..=n).step_by(2) {
            print!("{} ", a)
        }
    }
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
