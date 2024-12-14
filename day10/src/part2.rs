#![allow(dead_code, unused_variables)]

const RADIX: u32 = 10;

fn get_moves(
    map: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    height: u8,
) -> Option<Vec<(usize, usize)>> {
    let mut result = vec![];
    let next_height = height + 1;

    // Upwards movement
    if row > 0 && map[row - 1][col] == next_height {
        result.push((row - 1, col));
    }

    // Downwards movement
    if row < (map.len() - 1) && map[row + 1][col] == next_height {
        result.push((row + 1, col));
    }

    // Left movement
    if col > 0 && map[row][col - 1] == next_height {
        result.push((row, col - 1));
    }

    // Right movement
    if col < (map[0].len() - 1) && map[row][col + 1] == next_height {
        result.push((row, col + 1));
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}

fn count_trail_score(map: &Vec<Vec<u8>>, row: usize, col: usize, height: u8) -> usize {
    match get_moves(map, row, col, height) {
        Some(moves) if height < 8 => moves
            .into_iter()
            .map(|m| count_trail_score(map, m.0, m.1, height + 1))
            .sum(),
        Some(moves) => moves.len(),
        None => 0,
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut trailheads = vec![];
    let map: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .map(|(col, c)| {
                    if c == '0' {
                        trailheads.push((row, col));
                    }
                    c.to_digit(RADIX).unwrap() as u8
                })
                .collect()
        })
        .collect();

    let result = trailheads
        .into_iter()
        .map(|t| count_trail_score(&map, t.0, t.1, 0))
        .sum();
    Ok(result)
}
