#![allow(dead_code, unused_variables)]

use regex::Regex;

// use rayon::prelude::*;

pub fn run(input: &str) -> Result<usize, String> {
    let search = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let result = search
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum();
    Ok(result)
}
