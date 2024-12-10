#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

// use rayon::prelude::*;
const OBSTRUCTION: u8 = b'#';
const START: u8 = b'^';
const FLOOR: u8 = b'.';

#[derive(Clone, Debug)]
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

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

fn move_guard(map: &Vec<Vec<&u8>>, pos: &Position) -> Option<Position> {
    // Get next coordinate
    let (row, col) = match pos.direction {
        Direction::Up => {
            if pos.row > 0 {
                (pos.row - 1, pos.col)
            } else {
                return None;
            }
        }
        Direction::Left => {
            if pos.col > 0 {
                (pos.row, pos.col - 1)
            } else {
                return None;
            }
        }
        Direction::Right => {
            if pos.col < (map[0].len() - 1) {
                (pos.row, pos.col + 1)
            } else {
                return None;
            }
        }
        Direction::Down => {
            if pos.row < (map.len() - 1) {
                (pos.row + 1, pos.col)
            } else {
                return None;
            }
        }
    };
    if map[row][col] == &OBSTRUCTION {
        return Some(Position {
            row: pos.row,
            col: pos.col,
            direction: pos.direction.turn(),
        });
    }
    Some(Position {
        row,
        col,
        direction: pos.direction.clone(),
    })
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut pos = Some(Position {
        row: 0,
        col: 0,
        direction: Direction::Up,
    });
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(col, c)| {
                    if c == &START {
                        pos = Some(Position {
                            row,
                            col,
                            direction: Direction::Up,
                        })
                    }
                    c
                })
                .collect::<Vec<&u8>>()
        })
        .collect::<Vec<Vec<&u8>>>();

    let mut positions = HashSet::new();

    loop {
        match &pos {
            Some(next) => positions.insert((next.row, next.col)),
            None => break,
        };
        pos = move_guard(&map, &(pos.unwrap()));
        // println!("Position: {:?}", &pos);
    }

    Ok(positions.into_iter().count())
}
