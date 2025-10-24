use std::fmt::Debug;
use std::io;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    let n: u64 = read_inp();
    let mut sum = 0u64;
    for _ in 0..n - 1 {
        let a: u64 = read_token(stdin().lock());
        sum += a;
    }
    let full = (1 + n) * n / 2;
    let missing = full - sum;
    print!("{}", missing);
}

fn read_inp<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut stdin = io::stdin().lock();
    let mut line = String::new();
    stdin
        .read_line(&mut line)
        .expect("failed to read from stdin");
    line.trim().parse().expect("Not a number!")
}
fn read_token<T: FromStr>(mut reader: impl BufRead) -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut buf = Vec::new();
    reader.read_until(b' ', &mut buf).expect("failed to read from stdin");
    let str = String::from_utf8(buf).expect("not a valid UTF-8 string");
    str.trim().parse().expect("not a valid number")
}