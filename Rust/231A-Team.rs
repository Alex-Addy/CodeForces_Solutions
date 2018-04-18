
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let problems = get_numbers(&input)[0];
    let mut solutions = 0;

    for _ in 0..problems {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sureness: usize = get_numbers(&input).iter().sum();

        if sureness >= 2 {
            solutions += 1;
        }
    }
    
    println!("{}", solutions);
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

