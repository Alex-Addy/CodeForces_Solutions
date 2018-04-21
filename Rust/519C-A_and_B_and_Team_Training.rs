
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<isize> = get_numbers(&input);
    let n = nums[0]; // experienced members
    let m = nums[1]; // newbies

    let mut teams = 0;
    let mut exp = n;
    let mut newb = m;
    while exp > 0 && newb > 0 {
        if exp > newb {
            exp -= 2;
            newb -= 1;
            teams += 1;
        } else {
            exp -= 1;
            newb -= 2;
            teams += 1;
        }
    }

    if exp == -1 || newb == -1 {
        teams -= 1;
    }

    println!("{}", teams);
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

