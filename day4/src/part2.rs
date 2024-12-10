#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;
use regex::Regex;

pub fn run(input: &str) -> Result<usize, String> {
    let re = Regex::new(r"MS").unwrap();

    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut count = 0;

    for y in 1..(map.len() - 1) {
        for x in 1..(map[0].len() - 1) {
            if map[y][x] == b'A' {
                let diagonals = [
                    [[y - 1, x - 1], [y + 1, x + 1]],
                    [[y + 1, x + 1], [y - 1, x - 1]],
                    [[y + 1, x - 1], [y - 1, x + 1]],
                    [[y - 1, x + 1], [y + 1, x - 1]],
                ]
                .into_iter()
                .map(|[[y1, x1], [y2, x2]]| {
                    String::from_utf8(vec![map[y1][x1], map[y2][x2]]).unwrap()
                })
                .filter(|diag| diag == "MS")
                .count();
                if diagonals == 2 {
                    count = count + 1
                }
            }
        }
    }

    Ok(count)
}
