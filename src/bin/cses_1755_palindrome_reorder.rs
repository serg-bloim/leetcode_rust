#![allow(unused)]

use std::fmt::Debug;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut chars = [0u32; 256];
    let mut read = 0;
    for ch in stdin().lock().bytes() {
        if let Ok(ch) = ch {
            if  ch >= b'A' && ch <= b'Z' {
                chars[ch as usize] += 1;
                read += 1;
            }
        }else{
            break;
        }
    }
    let mut odd_ch = 0u8;
    let mut odd_chars = 0;

    for ch in 0..255u8 {
        let n = chars[ch as usize];
        if n % 2 == 1 {
            odd_ch = ch;
            odd_chars += 1;
        }
    }
    if odd_chars > 1 {
        println!("NO SOLUTION");
        return;
    }

    for ch in 0..255u8 {
        let n = chars[ch as usize];
        if ch != odd_ch {
            for _ in 0..n / 2 {
                stdout().lock().write_all(&[ch]).unwrap();
            }
        }
    }
    for _ in 0..chars[odd_ch as usize] {
        stdout().lock().write_all(&[odd_ch]).unwrap();
    }

    for ch in (0..255u8).rev() {
        let n = chars[ch as usize];
        if ch != odd_ch {
            for _ in 0..n / 2 {
                stdout().lock().write_all(&[ch]).unwrap();
            }
        }
    }
}
