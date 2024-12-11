#![allow(dead_code, unused_variables)]

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

struct Antenna {
    freq: char,
    row: usize,
    col: usize,
}

fn resonances(antennae: &Vec<Antenna>, height: usize, width: usize) -> HashSet<(i32, i32)> {
    antennae
        .iter()
        .permutations(2)
        .flat_map(|v| get_resonances(v[0], v[1], height, width))
        .collect()
}

fn get_resonances(a: &Antenna, b: &Antenna, height: usize, width: usize) -> Vec<(i32, i32)> {
    let row_diff = a.row as i32 - b.row as i32;
    let col_diff = a.col as i32 - b.col as i32;
    let mut resonances = vec![];

    for n in 0.. {
        let cur_row = a.row as i32 + (n * row_diff);
        let cur_col = a.col as i32 + (n * col_diff);
        if cur_row >= 0 && cur_col >= 0 && cur_row < height as i32 && cur_col < width as i32 {
            resonances.push((cur_row, cur_col));
        } else {
            break;
        }
    }
    resonances
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut antennae: HashMap<char, Vec<Antenna>> = HashMap::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    for (row, line) in input.lines().enumerate() {
        for (col, freq) in line.chars().enumerate() {
            let antenna = match freq {
                '.' => None,
                _ => Some(Antenna {
                    row: row.clone(),
                    col: col.clone(),
                    freq: freq.clone(),
                }),
            };
            if let Some(antenna) = antenna {
                match antennae.get_mut(&freq) {
                    Some(list) => list.push(antenna),
                    None => {
                        let mut list = vec![];
                        list.push(antenna);
                        antennae.insert(freq, list);
                    }
                }
            };
        }
    }
    let resonances = antennae
        .iter()
        .map(|(freq, antenna_list)| resonances(&antenna_list, height, width))
        .reduce(|mut acc, r| {
            acc.extend(r);
            acc
        })
        .unwrap()
        .len();
    Ok(resonances)
}
