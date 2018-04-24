
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
    let n = get_numbers::<usize>(&line)[0];
    line.clear();
    input.read_line(&mut line).unwrap();
    let arr = get_numbers::<usize>(&line);

    let min_i = arr.iter().position(|&v| v == 1).unwrap();
    let max_i = arr.iter().position(|&v| v == n).unwrap();

    let d_min = f64::abs(min_i as f64 - ((n-1) as f64 / 2.0)); // distance to center
    let d_max = f64::abs(max_i as f64 - ((n-1) as f64 / 2.0));
    
    // pick closest to center to move
    let d = if d_min < d_max {
        if min_i < max_i { // move min to 0
            max_i
        } else {
            (n-1)-max_i // move min to end
        }
    } else {
        if max_i < min_i {
            min_i
        } else {
            (n-1)-min_i
        }
    };

    println!("{}", d);
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

