#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

#[derive(PartialEq, Debug)]
enum Dir {
    Increasing,
    Decreasing,
    Broken,
}

#[derive(Debug)]
struct Report {
    levels: Vec<usize>,
    direction: Option<Dir>,
}

impl Report {
    fn new() -> Report {
        Report {
            levels: vec![],
            direction: None,
        }
    }

    fn add(mut self, level: usize) -> Report {
        let last = self.levels.last();
        match last {
            Some(last) => match self.direction {
                Some(Dir::Increasing) => {
                    if &level <= last || &level - last > 3 {
                        self.direction = Some(Dir::Broken);
                    }
                }
                Some(Dir::Decreasing) => {
                    if &level >= last || last - &level > 3 {
                        self.direction = Some(Dir::Broken);
                    }
                }
                Some(_) => {}
                None => {
                    if &level > last {
                        if &level - last <= 3 {
                            self.direction = Some(Dir::Increasing);
                        } else {
                            self.direction = Some(Dir::Broken);
                        }
                    } else if &level < last {
                        if last - level <= 3 {
                            self.direction = Some(Dir::Decreasing);
                        } else {
                            self.direction = Some(Dir::Broken);
                        }
                    } else {
                        self.direction = Some(Dir::Broken);
                    }
                }
            },
            None => {}
        }

        self.levels.push(level);
        self
    }

    fn is_safe(self) -> bool {
        self.direction != Some(Dir::Broken)
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let result = input
        .lines()
        .map(|line| {
            let mut r = Report::new();
            let mut levels = line.split(' ');
            while let Some(level) = levels.next() {
                r = r.add(level.parse().unwrap());
            }
            if r.is_safe() {
                1
            } else {
                0
            }
        })
        .sum();
    Ok(result)
}
