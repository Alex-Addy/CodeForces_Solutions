
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
    let n = get_numbers(&line)[0];
    line.clear(); input.read_line(&mut line).unwrap();
    let nums = get_numbers::<isize>(&line);

    let mut pos_indices = Vec::new();
    let mut neg_indices = Vec::new();
    let mut zero_indices = Vec::new();

    for i in 0..n {
        if nums[i] > 0 {
            pos_indices.push(i);
        } else if nums[i] < 0 {
            neg_indices.push(i);
        } else {
            zero_indices.push(i);
        }
    }

    assert!(neg_indices.len() > 0);
    assert!(zero_indices.len() > 0);

    let mut neg_set = Vec::new();
    let mut pos_set = Vec::new();
    let mut zero_set = Vec::new();
    neg_set.push(nums[neg_indices.pop().unwrap()]);

    if pos_indices.len() == 0 {
        if neg_indices.len() < 2 {
            println!("Cannot create positive set");
        }
        pos_set.push(nums[neg_indices.pop().unwrap()]);
        pos_set.push(nums[neg_indices.pop().unwrap()]);
    }
    let _ = pos_indices.drain(..).map(|ind| pos_set.push(nums[ind])).last();
    let _ = zero_indices.drain(..).map(|ind| zero_set.push(nums[ind])).last();
    let _ = neg_indices.drain(..).map(|ind| zero_set.push(nums[ind])).last();

    print_set(neg_set.as_slice());
    print_set(pos_set.as_slice());
    print_set(zero_set.as_slice());
}

fn print_set(set: &[isize]) {
    print!("{}", set.len());

    for i in 0..set.len() {
        print!(" {}", set[i]);
    }

    println!("");
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

