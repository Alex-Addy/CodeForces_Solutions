
#![allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let nums = get_numbers(&line);
    let num_verts = nums[0];
    let max_cats = nums[1];
    line.clear();
    input.read_line(&mut line).unwrap();
    let cats: Vec<u8> = get_numbers(&line);
    line.clear();

    let mut tree = Vec::with_capacity(num_verts);
    for i in 0..num_verts {
        tree.push(Node::new(i, cats[i] == 1));
    }

    for _ in 0..(num_verts-1) {
        line.clear();
        input.read_line(&mut line).unwrap();
        let edge: Vec<usize> = get_numbers(&line);
        tree[edge[1]-1].add_child(edge[0]-1); // add both ways, since we don't know which is the parent
        tree[edge[0]-1].add_child(edge[1]-1); // -1 as the given ids start from 1
    }

    // setup is done, time to solve
    //println!("{}", solve_recursively(&tree, max_cats, 0, 0, 0));
    println!("{}",  solve_iteratively(&tree, max_cats));
}

// solves problem recursively
// blows the stack in worst case scenarios making it not usable for solution
fn _solve_recursively(tree: &Vec<Node>, max_cats: usize, cats: usize, parent: usize, id: usize) -> usize {
    let node = &tree[id];
    if node.cat && cats == max_cats {
        return 0;
    }
    if node.children.len() == 1 && node.children[0] == parent { // leaf node
        return 1;
    }

    let cat_count = if node.cat { cats + 1 } else { 0 };
    let mut sum = 0;
    for i in 0..node.children.len() {
        if node.children[i] == parent {
            // guaranteed only one parent so this check is sufficient to prevent traversal loops
            continue;
        }

        sum += _solve_recursively(&tree, max_cats, cat_count, id, node.children[i]);
    }

    sum
}

fn solve_iteratively(tree: &Vec<Node>, max_cats: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let mut sum = 0;
    loop {
        let (parent, id, cats) = match queue.pop_front() {
            Some((p, i, c)) => (p, i, c),
            None => break,
        };
        let node = &tree[id];
        if cats == max_cats && node.cat {
            // go no further on this path
            continue;
        }
        if node.children.len() == 1 && node.children[0] == parent {
            sum += 1;
        }

        let cat_count = if node.cat { cats + 1 } else { 0 };
        for i in 0..node.children.len() {
            if node.children[i] == parent {
                continue;
            }
            queue.push_back((id, node.children[i], cat_count));
        }
    }

    sum
}

#[derive(Debug, PartialEq)]
struct Node {
    id: usize,
    cat: bool,
    children: Vec<usize>,
}

impl Node {
    fn new(id: usize, cat: bool) -> Node {
        Node {
            id: id,
            cat: cat,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, id: usize) {
        self.children.push(id);
    }
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

