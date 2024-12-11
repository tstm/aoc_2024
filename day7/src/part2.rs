#![allow(dead_code, unused_variables)]

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Ops {
    Add,
    Multiply,
    Concat,
}

struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn calc(&self) -> Option<usize> {
        let mut options = vec![];
        for i in 0..(self.numbers.len() - 1) {
            options.push([Ops::Add, Ops::Multiply, Ops::Concat])
        }

        for operations in options.into_iter().multi_cartesian_product() {
            let mut result = self.numbers[0];
            for (i, op) in operations.into_iter().enumerate() {
                match op {
                    Ops::Add => result = result + self.numbers[i + 1],
                    Ops::Multiply => result = result * self.numbers[i + 1],
                    Ops::Concat => {
                        result = Equation::concat(result, self.numbers[i + 1]);
                    }
                }
            }
            if result == self.result {
                return Some(result);
            }
        }
        None
    }

    fn concat(a: usize, b: usize) -> usize {
        a * 10usize.pow(b.ilog10() + 1) + b
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let (result, rest) = line.split_once(':').unwrap();
            let result: usize = result.parse().unwrap();
            let numbers = rest
                .trim()
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Equation { result, numbers }
        })
        .collect();

    let result = equations.into_par_iter().flat_map(|e| e.calc()).sum();

    Ok(result)
}
