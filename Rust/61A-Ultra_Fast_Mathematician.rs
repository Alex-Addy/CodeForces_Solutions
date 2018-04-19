
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut first = String::new();
    let mut second = String::new();
    io::stdin().read_line(&mut first).unwrap();
    io::stdin().read_line(&mut second).unwrap();
    let fir: Vec<char> = first.chars().filter(|c| c.is_digit(2)).collect();
    let sec: Vec<char> = second.chars().filter(|c| c.is_digit(2)).collect();
    let mut out = vec!['_'; fir.len()];

    for i in 0..fir.len() {
        if fir[i] == sec[i] {
            out[i] = '0';
        } else {
            out[i] = '1';
        }
    }

    println!("{}", out.iter().collect::<String>());
}

