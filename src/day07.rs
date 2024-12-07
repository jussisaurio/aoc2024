use crate::util::aoc_read_day_lines;
use memmap::Mmap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

trait Concat {
    fn concat(&self, other: usize) -> usize;
}

impl Concat for usize {
    /// eg 12.concat(135) == 12135
    fn concat(&self, other: usize) -> usize {
        let num_digits = (other as f64).log10().floor() as usize + 1;
        self * 10usize.pow(num_digits as u32) + other
    }
}

fn parse_line(target: usize, operands: &[usize], is_part2: bool) -> usize {
    let mut stack = Vec::with_capacity(100);

    if operands.len() > 0 {
        stack.push((operands[0], 0));
    }
    while let Some((n, i)) = stack.pop() {
        let is_last = i == operands.len() - 1;

        if is_last && n == target {
            return target;
        }

        if !is_last {
            let addition = n + operands[i + 1];
            if addition <= target {
                stack.push((addition, i + 1));
            }
            let multiplication = n * operands[i + 1];
            if multiplication <= target {
                stack.push((multiplication, i + 1));
            }
            if is_part2 {
                let concatenation = n.concat(operands[i + 1]);
                if concatenation <= target {
                    stack.push((concatenation, i + 1));
                }
            }
        }
    }

    0
}

pub fn day7_part1() -> usize {
    let lines = aoc_read_day_lines(7);
    lines
        .par_iter()
        .map(|line| {
            let (target, operands) = line.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let operands: Vec<_> = operands.split(' ').map(|s| s.parse().unwrap()).collect();
            parse_line(target, &operands, false)
        })
        .sum()
}

pub fn day7_part2() -> usize {
    let lines = aoc_read_day_lines(7);
    lines
        .par_iter()
        .map(|line| {
            let (target, operands) = line.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let operands: Vec<_> = operands.split(' ').map(|s| s.parse().unwrap()).collect();
            parse_line(target, &operands, true)
        })
        .sum()
}
