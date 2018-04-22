
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
    let nums = get_numbers::<usize>(&line);
    let num_markets = nums[0];
    let kilos = nums[1];

    let mut cheapest = std::f64::MAX;
    for _ in 0..num_markets {
        line.clear();
        input.read_line(&mut line).unwrap();
        let nums = get_numbers::<f64>(&line);
        cheapest = f64::min(cheapest, nums[0]/nums[1]);
    }

    println!("{}", cheapest * kilos as f64);
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

