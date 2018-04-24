
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let lucky: usize = get_numbers(&line)[0];

    let mut cur = 4;
    for i in 1.. {
        if lucky == cur {
            println!("{}", i);
            break;
        }

        cur = next(cur);
    }
}

const C4: u8 = '4' as u8;
const C7: u8 = '7' as u8;

// return next lucky number, assumes n is lucky
fn next(n: usize) -> usize {
    let mut digits = format!("{}", n);
    let mut s = Vec::from(digits);
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s[i] == C4 {
            s[i] = C7;
            break;
        }
        s[i] = C4;
    }

    let mut n = str::from_utf8(&s).unwrap().parse().unwrap();
    if i == 0 && s[0] == C4 { // got all fours, need to add new digit
        n += 4 * 10usize.pow(s.len() as u32);
    }

    n
}

/*
 * Utility Functions
 */

// get numbers from line seperated by spaces
fn get_numbers<T>(line: &str) -> Vec<T>
    where T: FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line.split_whitespace()
        .map(|chunk| chunk.parse::<T>().expect("failed to parse"))
        .collect()
}

