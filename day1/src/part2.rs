#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

pub fn run(input: &str) -> Result<usize, String> {
    let (mut left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("  ").unwrap();
            let left = left.trim().parse().unwrap();
            let right = right.trim().parse().unwrap();
            (left, right)
        })
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .unzip();
    left.sort();
    // right.sort();

    Ok(left
        .chunk_by(|a, b| a == b)
        .map(|chunk| {
            let value = chunk[0];
            let match_count = right.iter().filter(|&n| *n == value).count();
            value * match_count * chunk.len()
        })
        .sum())
}
