
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::iter;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let center = get_numbers(&input)[0];
    
    for i in 0..center {
        println!("{}", make_line(i, center));
    }
    println!("{}", make_line(center, center));
    for inv in 0..center {
        let i = center - inv - 1; // lack of reverse iterators can be annoying
        println!("{}", make_line(i, center));
    }
}

fn make_line(row: isize, max: isize) -> String {
    let offset = (max - row).abs();
    let center = (max - offset) as usize;
    let preceding_spaces = (offset * 2) as usize;
    let mut line: Vec<char> = iter::repeat(' ').take(preceding_spaces).collect();
    for i in 0..(center+1) {
        line.push(digit_to_char(i as u8));
        line.push(' ');
    }
    for inv in 0..center {
        let i = center - inv - 1;
        line.push(digit_to_char(i as u8));
        line.push(' ');
    }
    line.pop(); // pop trailing space

    line.iter().collect()
}

// get numbers from line seperated by spaces
fn get_numbers<T>(line: &str) -> Vec<T>
    where T: FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.split_whitespace()
        .map(|chunk| chunk.parse::<T>().expect("failed to parse"))
        .collect()
}

// convert a digit value 0..10 to its representative ascii value
// e.g. 0 -> '0', 1 -> '1', ..., 9 -> '9'
// panics if d is >= 10
fn digit_to_char(d: u8) -> char {
    if d >= 10 {
        panic!("cannot convert digit >= 10 to a single char");
    }

    char::from(d + 48) // '0' == 48
}
