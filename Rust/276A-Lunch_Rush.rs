
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
    let nums = get_numbers::<isize>(&line);
    let n = nums[0] as usize;
    let k = nums[1];

    let mut joys = vec![0isize; n];
    let mut times = vec![0isize; n];

    for i in 0..n {
        line.clear();
        input.read_line(&mut line).unwrap();
        let nums = get_numbers(&line);
        joys[i] = nums[0];
        times[i] = nums[1];
    }

    let mut max = std::isize::MIN;
    for i in 0..n {
        let val = if times[i] <= k {
            joys[i]
        } else {
            joys[i] - (times[i] - k)
        };
        max = std::cmp::max(max, val);
    }

    println!("{}", max);
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

