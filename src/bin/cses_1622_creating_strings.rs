use std::fmt::Debug;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() {
    let str = read_single::<String>();
    let mut vec: Vec<_> = str.chars().collect();
    process(&mut vec[..]);
}
fn process(str: &mut [char]) {
    str.sort();
    println!("{}", calc_perms(str));
    for c in str.iter() {
        print!("{}", c);
    }
    println!();
    loop {
        let right_unsorted = str.windows(2).rposition(|w| w[0] < w[1]);
        if let Some(right_unsorted) = right_unsorted {
            let insert_elem = str[right_unsorted];
            let remove_pos = right_unsorted + str[right_unsorted..]
                .iter()
                .rposition(|c| *c > insert_elem)
                .expect(
                    "No way there's no bigger element in the end of the str compared to beginning",
                );
            str.swap(right_unsorted, remove_pos);
            str[right_unsorted + 1..].sort();

            for c in str.iter() {
                print!("{}", c);
            }
            println!()
        } else {
            break;
        }
    }
}
fn calc_perms(str: &[char]) -> u32 {
    let mut rep_factor = 1;
    let mut last_ch = str.first().expect("Empty str");
    let mut reps = 0;
    for ch in str.iter() {
        if ch == last_ch {
            reps += 1;
        } else {
            rep_factor *= factorial(reps);
            last_ch = ch;
            reps = 1;
        }
    }
    rep_factor *= factorial(reps);
    factorial(str.len() as u32) / rep_factor
}

fn factorial(n: u32) -> u32 {
    (1..=n).reduce(|acc, n| acc * n).unwrap_or(1)
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
    let mut str: Vec<_> = "abc".chars().collect();
    process(&mut str[..]);
}
#[test]
fn test2() {
    let x = &"aab".chars().collect::<Vec<_>>()[..];
    assert_eq!(calc_perms(&x), 3);
    let x = &"abc".chars().collect::<Vec<_>>()[..];
    assert_eq!(calc_perms(&x), 6);
    let x = &"aaaaaaaa".chars().collect::<Vec<_>>()[..];
    assert_eq!(calc_perms(&x), 1);
}

// fn test(str_slice :&[char]){
//     let mut str_slice_mutable = str_slice;
//     str_slice_mutable.swap(1, 2);
//     // str_slice.swap(1, 2);
// }
