#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

// use rayon::prelude::*;
const OBSTRUCTION: u8 = b'#';
const START: u8 = b'^';
const FLOOR: u8 = b'.';

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

struct Status<'a> {
    pos: Position,
    map: &'a Vec<Vec<u8>>,
}

impl<'a> Status<'a> {
    fn new(pos: Position, map: &'a Vec<Vec<u8>>) -> Status<'a> {
        Status { pos, map: &map }
    }
}

impl Position {
    fn get_coords(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<'a> Iterator for Status<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        // Get next coordinate
        let (row, col) = match self.pos.direction {
            Direction::Up => {
                if self.pos.row > 0 {
                    (self.pos.row - 1, self.pos.col)
                } else {
                    // eprintln!("Found top");
                    return None;
                }
            }
            Direction::Left => {
                if self.pos.col > 0 {
                    (self.pos.row, self.pos.col - 1)
                } else {
                    // eprintln!("Found left edge");
                    return None;
                }
            }
            Direction::Right => {
                if self.pos.col < (self.map[0].len() - 1) {
                    (self.pos.row, self.pos.col + 1)
                } else {
                    // eprintln!("Found right edge");
                    return None;
                }
            }
            Direction::Down => {
                if self.pos.row < (self.map.len() - 1) {
                    (self.pos.row + 1, self.pos.col)
                } else {
                    // eprintln!("Found bottom");
                    return None;
                }
            }
        };
        if self.map[row][col] == OBSTRUCTION {
            self.pos.direction = self.pos.direction.turn();
            return Some(self.pos.clone());
        }
        self.pos.row = row;
        self.pos.col = col;
        Some(self.pos.clone())
    }
}

impl<'a> std::fmt::Display for Status<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if col == self.pos.col && row == self.pos.row {
                    write!(f, "*")?;
                } else {
                    write!(
                        f,
                        "{}",
                        String::from_utf8(vec!(self.map[row][col])).unwrap()
                    )?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let (mut start_row, mut start_col) = (0, 0);
    let mut wall_positions = HashSet::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(col, c)| {
                    if c == &START {
                        start_row = row;
                        start_col = col;
                    } else if c == &OBSTRUCTION {
                        wall_positions.insert((row, col));
                    }
                    c.clone()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let start_status = Status {
        pos: Position {
            row: start_row,
            col: start_col,
            direction: Direction::Up,
        },
        map: &map,
    };

    let mut positions = HashSet::new();

    let count = start_status
        .into_iter()
        .map(|p| {
            positions.insert(p.clone());

            // Check if we are not against a wall
            if let Some(n) = Status::new(p.clone(), &map).into_iter().next() {
                let wall_coords = n.get_coords();
                if !wall_positions.contains(&wall_coords) {
                    wall_positions.insert(wall_coords);

                    // Set up the inner map with the wall
                    let mut inner_map = map.clone();
                    inner_map[wall_coords.0][wall_coords.1] = b'#';
                    let hit_wall = Status::new(p, &inner_map);

                    let mut inner_positions = HashSet::new();

                    let mut inner_count = 0;
                    for inner in hit_wall {
                        if inner_positions.contains(&inner) || positions.contains(&inner) {
                            // eprintln!("Broke infinite loop");
                            inner_count = 1;
                            break;
                        }
                        inner_positions.insert(inner.clone());
                    }
                    inner_count
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    Ok(count)
}
