
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
    let nums = get_numbers(&line);
    let num_prisoners = nums[0];
    let max_severity = nums[1];
    let choose = nums[2];

    line.clear(); input.read_line(&mut line).unwrap();
    let prisoners = get_numbers::<usize>(&line);

    let mut choices = vec![0usize; num_prisoners];
    for i in 0..num_prisoners {
        if prisoners[i] <= max_severity {
            if i == 0 {
                choices[0] = 1;
            } else {
                choices[i] = choices[i-1] + 1;
            }
        } else {
            choices[i] = 0;
        }
    }

    let ways: usize = choices.iter().map(|&v| if v >= choose { 1 } else { 0 }).sum();

    println!("{}", ways);
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

