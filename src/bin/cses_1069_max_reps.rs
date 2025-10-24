use std::cmp::max;
use std::fmt::Debug;
use std::io::{Read, stdin};

fn main() {
    let mut last_ch = b'?';
    let mut max_reps = 0;
    let mut reps = 0;
    for b in stdin().lock().bytes() {
        let b = b.unwrap();
        if b == last_ch {
            reps += 1;
        } else {
            last_ch = b;
            if reps > max_reps {
                max_reps = reps;
            }
            reps = 1;
        }
    }
    let res = max(max_reps, reps);
    println!("{}", res);
}
