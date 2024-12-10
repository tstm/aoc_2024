#![allow(dead_code, unused_variables)]

use regex::Regex;

// use rayon::prelude::*;

pub fn run(input: &str) -> Result<usize, String> {
    let search = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let donts = Regex::new(r"don't\(\)").unwrap();
    let dos = Regex::new(r"do\(\)").unwrap();

    let mut clean = "".to_string();

    let mut offset_start = 0;
    let offset_end = donts.shortest_match(input).unwrap();
    clean = clean + &input[offset_start..offset_end];

    offset_start = offset_end;
    while let Some(mut do_start) = dos.shortest_match(&input[offset_start..]) {
        do_start = do_start + offset_start;

        if let Some(mut do_end) = donts.shortest_match(&input[do_start..]) {
            do_end = do_start + do_end;
            clean = clean + &input[do_start..do_end];
            offset_start = do_end;
        } else {
            clean = clean + &input[do_start..];
            offset_start = input.len();
        }
    }

    let result = search
        .captures_iter(&clean)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum();
    Ok(result)
}
