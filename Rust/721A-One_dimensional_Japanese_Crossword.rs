
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_squares = get_numbers(&input)[0];
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let squares: Vec<char> = input.chars().collect();

    let mut numbers = Vec::new();
    let mut n = 0;
    for i in 0..num_squares {
        if squares[i] == 'B' {
            n += 1;
        } else if n > 0 {
            numbers.push(n);
            n = 0;
        }
    }
    if n > 0 {
        numbers.push(n);
    }

    if numbers.len() == 0 {
        println!("{}", 0);
        return;
    }
    println!("{}", numbers.len());
    if numbers.len() == 1 {
        println!("{}", numbers[0]);
        return;
    }

    for i in 0..(numbers.len()-1) {
        print!("{} ", numbers[i])
    }
    println!("{}", numbers[numbers.len()-1]);
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

