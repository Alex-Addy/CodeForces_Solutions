
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let nums = get_numbers::<u64>(&line);
    let n = nums[0];
    let k = nums[1] - 1; // k is numbered from 1

    let length = u64::pow(2, n as u32) - 1;

    let mut val = n;
    let mut ind = length / 2;
    let mut lower = 0;
    let mut upper = length;

    while ind != k {
        val = val - 1;
        if ind > k {
            upper = ind;
            ind = (lower + upper) / 2;
        } else {
            lower = ind;
            ind = (lower + upper) / 2;
        }
    }

    println!("{}", val);
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

