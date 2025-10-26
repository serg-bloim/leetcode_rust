use std::io::stdin;

mod my_inp;
fn main() {
    let mut parser = my_inp::Parser::new(stdin().lock());
    let mut sum= 0;
    for n in parser.read_separated::<i32>(b' '){
        sum += n;
    }
    println!("sum: {}", sum);
}