#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;
use regex::Regex;

fn transpose(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = input.len();
    let cols = input[0].len();
    (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect()
}

fn reverse(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|line| {
            let mut row = line.clone();
            row.reverse();
            row
        })
        .collect()
}

fn rotate_45_ccw(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = input.len();
    let cols = input[0].len();

    let mut result = vec![];
    for i in 0..cols {
        let mut row = vec![];
        let (mut a, mut b) = (0, 0);
        while let Some(r) = input.get(a) {
            if let Some(x) = r.get(i + b) {
                row.push(x.clone());
            }
            a = a + 1;
            b = b + 1;
        }
        result.push(row);
    }
    for i in 1..rows {
        let mut row = vec![];
        let (mut a, mut b) = (0, 0);
        while let Some(r) = input.get(i + a) {
            if let Some(x) = r.get(b) {
                row.push(x.clone());
            }
            a = a + 1;
            b = b + 1;
        }
        result.push(row);
    }

    result
}

pub fn run(input: &str) -> Result<usize, String> {
    let re = Regex::new(r"XMAS").unwrap();

    let mut original = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let topbottom = transpose(&original);
    let rightleft = reverse(&original);
    let bottomtop = reverse(&topbottom);

    let ccw_45 = rotate_45_ccw(&original);
    let ccw_135 = rotate_45_ccw(&bottomtop);
    let ccw_225 = reverse(&ccw_45);
    let ccw_315 = reverse(&ccw_135);

    original.extend(bottomtop);
    original.extend(rightleft);
    original.extend(topbottom);
    original.extend(ccw_45);
    original.extend(ccw_135);
    original.extend(ccw_225);
    original.extend(ccw_315);

    let mut count = 0;
    for line in original {
        // println!("{}", std::str::from_utf8(&line).unwrap());
        count = count + re.find_iter(&String::from_utf8(line).unwrap()).count();
    }

    Ok(count)
}
