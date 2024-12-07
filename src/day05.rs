use crate::util::aoc_read_day_lines;
use std::collections::{HashMap, HashSet};

// xx|yy
#[derive(Debug)]
struct Rule {
    fst: usize,
    snd: usize,
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let parts: [usize; 2] = s
            .split('|')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        Rule {
            fst: parts[0],
            snd: parts[1],
        }
    }
}

type Update = Vec<usize>;

pub fn day5_part1() -> usize {
    let lines = aoc_read_day_lines(5);

    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    // read until blank line to get rules, one per line
    let mut i = 0;
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let rule: Rule = Rule::from_str(&line);
        rules.push(rule);
        i += 1;
    }

    // rest are commaseparated updates
    for line in lines.iter().skip(i + 1) {
        let update: Update = line
            .split(',')
            .map(|s| s.parse().expect("update"))
            .collect();
        updates.push(update);
    }

    let mut valids = Vec::new();
    for u in updates.iter() {
        let mut valid = true;
        let positions_hashmap: HashMap<usize, usize> = u
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as usize))
            .collect();
        for r in rules.iter() {
            if !positions_hashmap.contains_key(&r.fst) || !positions_hashmap.contains_key(&r.snd) {
                continue;
            }
            let fst_pos = positions_hashmap[&r.fst];
            let snd_pos = positions_hashmap[&r.snd];
            if fst_pos > snd_pos {
                valid = false;
                break;
            }
        }
        if valid {
            valids.push(u);
        }
    }

    let sum_of_middle_nums: usize = valids.iter().map(|u| u[u.len() / 2]).sum();
    sum_of_middle_nums
}

pub fn day5_part2() -> usize {
    let lines = aoc_read_day_lines(5);
    let rules: Vec<Rule> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| Rule::from_str(line))
        .collect();
    let mut updates: Vec<Update> = lines
        .iter()
        .skip(rules.len() + 1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().expect("update"))
                .collect()
        })
        .collect();
    let mut invalids: Vec<_> = updates
        .iter_mut()
        .filter(|u| {
            rules.iter().any(|r| {
                let fst_pos = u.iter().position(|&x| x == r.fst);
                let snd_pos = u.iter().position(|&x| x == r.snd);

                match (fst_pos, snd_pos) {
                    (Some(fst), Some(snd)) => fst >= snd,
                    _ => false,
                }
            })
        })
        .collect();

    for invalid in invalids.iter_mut() {
        invalid.sort_by(|a, b| {
            let should = should_swap(*a, *b, &rules);
            should.cmp(&!should)
        });
    }

    let sum_of_middle_nums: usize = invalids.iter().map(|u| u[u.len() / 2]).sum();
    sum_of_middle_nums
}

fn should_swap(a: usize, b: usize, rules: &Vec<Rule>) -> bool {
    for r in rules.iter() {
        if r.fst == b && r.snd == a {
            return true;
        }
    }
    false
}
