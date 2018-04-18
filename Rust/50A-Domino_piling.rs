
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers = get_numbers(&input);
    let M = numbers[0];
    let N = numbers[1];
    println!("{}", cover(M, N));
}

fn cover(cols: u8, rows: u8) -> u8 {
    // start with laying them flat
    let flat = ((cols - (cols % 2)) / 2) * rows;
    
    let tall = if cols % 2 == 1 {
        // there is a leftover column, place dominos tall
        rows / 2
    } else {
        0
    };

    flat + tall
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

