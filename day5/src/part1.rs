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
            .all(|b| update.iter().position(|x| &x == b).unwrap() > index)
            && after
                .iter()
                .all(|a| update.iter().position(|x| &x == a).unwrap() < index)
    }
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
        let update: Vec<usize> = line
            .split(",")
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        let validated = update
            .iter()
            .enumerate()
            .map(|(index, u)| match rules.get(u) {
                Some(rule) => rule.test(&update, index),
                None => true,
            })
            .all(|x| x == true);
        if validated {
            eprintln!("{:?}", update);
            sum = sum + update[update.len() / 2]
        }
    }

    Ok(sum)
}
