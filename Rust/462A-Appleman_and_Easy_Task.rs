
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
    let mut board: Vec<Vec<char>> = Vec::with_capacity(n);
    
    for _ in 0..n {
        line.clear();
        input.read_line(&mut line).unwrap();
        board.push(line.chars().filter(|c| c.is_alphabetic()).collect());
    }

    if are_cells_even(&board) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn are_cells_even(board: &Vec<Vec<char>>) -> bool {
    for i in 0..board.len() {
        for k in 0..board.len() {
            if is_even(&board, i, k) == false {
                return false;
            }
        }
    }

    true
}

fn is_even(board: &Vec<Vec<char>>, i: usize, k: usize) -> bool {
    let mut count = 0;
    // check up
    if i > 0 {
        if board[i-1][k] == 'o' {
            count += 1;
        }
    }
    // check left
    if k > 0 {
        if board[i][k-1] == 'o' {
            count += 1;
        }
    }
    // check right
    if let Some(cell) = board[i].get(k+1) {
        if *cell == 'o' {
            count += 1;
        }
    }
    // check down
    if let Some(row) = board.get(i+1) {
        if row[k] == 'o' {
            count += 1;
        }
    }

    count % 2 == 0
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

