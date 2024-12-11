#![allow(dead_code, unused_variables)]

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

struct Antenna {
    freq: char,
    row: usize,
    col: usize,
}

fn resonances(
    antennae: &Vec<Antenna>,
    antenna_locations: &HashSet<(i32, i32)>,
    height: usize,
    width: usize,
) -> HashSet<(i32, i32)> {
    antennae
        .iter()
        .permutations(2)
        .map(|v| get_resonance(v[0], v[1]))
        // Check they are within the map
        .filter(|(row, col)| {
            row >= &0 && col >= &0 && row < &(height as i32) && col < &(width as i32)
        })
        .filter(|tuple| !antenna_locations.contains(&tuple))
        .collect()
}

fn get_resonance(a: &Antenna, b: &Antenna) -> (i32, i32) {
    let row_diff = a.row as i32 - b.row as i32;
    let col_diff = a.col as i32 - b.col as i32;
    (a.row as i32 + row_diff, a.col as i32 + col_diff)
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut antennae: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut antenna_locations: HashSet<(i32, i32)> = HashSet::new();
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
                antenna_locations.insert((row.clone() as i32, col.clone() as i32));
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
        .map(|(freq, antenna_list)| {
            resonances(&antenna_list, &antenna_locations, height, width).len()
        })
        .sum();
    Ok(resonances)
}
