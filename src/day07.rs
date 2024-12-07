use crate::util::aoc_mmap_day_input;
use memmap::Mmap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub struct Parser {
    mmap: Mmap,
    input: &'static [u8],
    offset: usize,
    is_part2: bool,
}

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

impl Parser {
    pub fn new((mmap, input): (Mmap, &'static [u8]), is_part2: bool) -> Self {
        Self {
            mmap,
            input,
            offset: 0,
            is_part2,
        }
    }

    fn next(&mut self) {
        self.offset += 1;
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.offset).copied()
    }

    fn parse_operands_of_line(&mut self) -> (usize, Vec<usize>) {
        let target = self.parse_number();
        self.next(); // skip colon
        self.next(); // skip space
        let mut operands = Vec::with_capacity(20);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                operands.push(self.parse_number());
            } else if c == b' ' {
                self.next();
            } else {
                assert!(c == b'\n');
                self.next();
                break;
            }
        }
        (target, operands)
    }

    fn parse(&mut self) -> usize {
        let mut operands_lines = Vec::with_capacity(100);
        while self.peek().is_some() {
            operands_lines.push(self.parse_operands_of_line());
        }
        operands_lines
            .par_iter()
            .map(|(target, operands)| parse_line(*target, operands, self.is_part2))
            .sum()
    }

    fn parse_number(&mut self) -> usize {
        let mut num = None;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                if let Some(n) = num {
                    num = Some(n * 10 + (c - b'0') as usize);
                } else {
                    num = Some((c - b'0') as usize);
                }
                self.next();
            } else {
                break;
            }
        }
        num.unwrap()
    }
}

pub fn day7_part1() -> usize {
    let (mmap, buf) = aoc_mmap_day_input(7);
    let mut parser = Parser::new((mmap, buf), false);
    parser.parse()
}

pub fn day7_part2() -> usize {
    let (mmap, buf) = aoc_mmap_day_input(7);
    let mut parser = Parser::new((mmap, buf), true);
    parser.parse()
}
