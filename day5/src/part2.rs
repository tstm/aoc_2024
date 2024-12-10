#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    before: Vec<usize>,
    after: Vec<usize>,
}

impl Rule {
    fn test(&self, update: &Vec<usize>, index: usize) -> bool {
        let before: Vec<&usize> = self.before.iter().filter(|b| update.contains(b)).collect();
        let after: Vec<&usize> = self.after.iter().filter(|a| update.contains(a)).collect();
        before
            .iter()
            .all(|b| update.iter().position(|x| &x == b).unwrap() >= index)
            && after
                .iter()
                .all(|a| update.iter().position(|x| &x == a).unwrap() <= index)
    }

    fn fix(&self, update: &Vec<usize>, num: usize) -> Option<Vec<usize>> {
        let before: Vec<&usize> = self.before.iter().filter(|b| update.contains(b)).collect();
        let after: Vec<&usize> = self.after.iter().filter(|a| update.contains(a)).collect();

        let mut result = update.clone();

        for i in 0..(result.len() + 1) {
            result.insert(i, num);
            if self.test(&result, i) {
                return Some(result);
            } else {
                // eprintln!(
                //     "Rule {:?} {:?} failed for {} index {} update: {:?}",
                //     before, after, num, i, result
                // );
                result.remove(i);
            }
        }
        None
    }
}

fn fix(rules: &HashMap<usize, Rule>, update: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![update[0]];

    for i in 1..update.len() {
        match rules.get(&update[i]) {
            Some(rule) => {
                result = rule.fix(&result, update[i]).unwrap();
            }
            None => {}
        }
    }
    result
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut line_iter = input.lines();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    // let mut updates = vec![];

    while let Some(line) = line_iter.next() {
        if line == "" {
            break;
        };
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();

        match rules.get_mut(&a) {
            Some(rule) => rule.before.push(b),
            None => {
                let rule = Rule {
                    before: vec![b],
                    after: vec![],
                };
                rules.insert(a, rule);
            }
        }

        match rules.get_mut(&b) {
            Some(rule) => rule.after.push(a),
            None => {
                let rule = Rule {
                    before: vec![],
                    after: vec![a],
                };
                rules.insert(b, rule);
            }
        }
    }

    // eprintln!("Rules: {:?}", rules);

    let mut sum = 0;

    while let Some(line) = line_iter.next() {
        let mut update: Vec<usize> = line
            .split(",")
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        let mut fixed = false;
        for i in 0..update.len() {
            match rules.get(&update[i]) {
                Some(rule) => {
                    if rule.test(&update, i) {
                    } else {
                        fixed = true;
                        update = fix(&rules, &update);
                    }
                }
                None => {}
            }
        }
        if fixed {
            sum = sum + update[update.len() / 2];
        }
    }

    Ok(sum)
}
