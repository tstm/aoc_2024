#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

fn abs_difference<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

pub fn run(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("  ").unwrap();
            let left: i32 = left.trim().parse().unwrap();
            let right: i32 = right.trim().parse().unwrap();
            (left, right)
        })
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .unzip();
    left.sort();
    right.sort();

    let difference = 0;
    Ok(left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| abs_difference(a, b))
        .sum())
}
