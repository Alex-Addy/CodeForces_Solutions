
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums = get_numbers(&input);
    let children = nums[0];
    let ticks = nums[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", solve(&input, children, ticks));
}

fn solve(queue: &str, num_children: usize, ticks: usize) -> String {
    let mut line: Vec<char> = queue.chars().filter(|c| c.is_alphanumeric()).collect();
    assert_eq!(line.len(), num_children);
    let mut line2 = vec!['0'; line.len()];
    
    for t in 0..ticks {
        let mut i = 0;
        while i < (num_children-1) {
            if t % 2 == 0 { // on even ticks place into line2
                if line[i] == 'B' && line[i+1] == 'G' {
                    line2[i] = 'G';
                    line2[i+1] = 'B';
                    i += 1;
                } else {
                    line2[i] = line[i];
                }
            } else { // on odd ticks place into line
                if line2[i] == 'B' && line2[i+1] == 'G' {
                    line[i] = 'G';
                    line[i+1] = 'B';
                    i += 1;
                } else {
                    line[i] = line2[i];
                }
            }
            i += 1;
        }
        if i == (num_children-1) { // didn't swap last position
            if t % 2 == 0 {
                line2[i] = line[i];
            } else {
                line[i] = line2[i];
            }
        }
    }

    if ticks % 2 == 1 {
        line2
    } else {
        line
    }.iter().collect()
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

