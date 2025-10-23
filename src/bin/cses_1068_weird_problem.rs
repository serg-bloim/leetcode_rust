use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut n: i64 = read_inp();
    let mut stdout = io::stdout();
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }
    print!("{}", n);
    stdout.flush().unwrap();
}

fn read_inp<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("failed to read from stdin");
    line.trim().parse().expect("Not a number!")
}
