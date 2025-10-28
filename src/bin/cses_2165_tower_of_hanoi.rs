use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    let disks = read_single();
    println!("{}", get_moves(disks));
    move_deep(disks, 1, 3, 2);
}

fn get_moves(disks: u32) -> u32 {
    2u32.pow(disks) - 1
}

fn move_deep(disk: u32, src: usize, dst: usize, buffer: usize) {
    if disk > 1 {
        move_deep(disk - 1, src, buffer, dst);
    }
    move_direct(src, dst);
    if disk > 1 {
        move_deep(disk - 1, buffer, dst, src);
    }
}

fn move_direct(from: usize, to: usize) {
    println!("{} {}", from, to);
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

#[test]
fn test1() {
    move_deep(3, 1, 3, 2)
}

#[test]
fn test2() {
    assert_eq!(get_moves(3), 7);
    assert_eq!(get_moves(5), 31);
    assert_eq!(get_moves(8), 255);
}
