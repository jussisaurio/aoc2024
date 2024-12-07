use crate::util::aoc_mmap_day_input;
use memmap::Mmap;
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

    fn parse_line(&mut self, operands: &mut Vec<usize>, stack: &mut Vec<(usize, usize)>) -> usize {
        let target = self.parse_number();
        self.next(); // skip colon
        self.next(); // skip space
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

        if operands.len() > 0 {
            stack.push((operands[0], 0));
        }
        while let Some((n, i)) = stack.pop() {
            if n > target {
                continue;
            }
            let is_last = i == operands.len() - 1;

            if is_last && n == target {
                return target;
            }

            if !is_last {
                stack.push((n + operands[i + 1], i + 1));
                stack.push((n * operands[i + 1], i + 1));
                if self.is_part2 {
                    stack.push((n.concat(operands[i + 1]), i + 1))
                }
            }
        }

        0
    }

    fn parse(&mut self) -> usize {
        let mut sum = 0;
        let mut operands = Vec::with_capacity(20);
        let mut stack = Vec::with_capacity(100);
        while self.peek().is_some() {
            sum += self.parse_line(&mut operands, &mut stack);
            operands.clear();
            stack.clear();
        }
        sum
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
