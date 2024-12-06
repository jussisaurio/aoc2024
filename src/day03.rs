use memmap::Mmap;

use crate::util::aoc_mmap_day_input;

#[derive(Debug, Clone, Copy)]
pub struct Mul {
    lhs: usize,
    rhs: usize,
}

#[derive(Debug)]
pub struct Parser {
    offset: usize,
    mmap: Mmap,
    input: &'static [u8],
    result: usize,
    lookahead_start: usize,
    mul_disabled: bool,
    feature_do_dont: bool,
}

impl Parser {
    pub fn new(mmap: Mmap, input: &'static [u8], feature_do_dont: bool) -> Self {
        Self {
            offset: 0,
            mmap,
            input,
            result: 0,
            lookahead_start: 0,
            mul_disabled: false,
            feature_do_dont,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        if self.offset >= self.input.len() {
            None
        } else {
            Some(self.input[self.offset])
        }
    }

    pub fn next(&mut self) {
        self.offset += 1;
    }

    pub fn savepoint(&mut self) {
        self.lookahead_start = self.offset;
    }

    pub fn retreat(&mut self) {
        self.offset = self.lookahead_start;
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        while self.peek().is_some() {
            match self.parse_dodont() {
                Ok(()) => continue,
                Err(()) => {}
            }
            match self.parse_mul() {
                Ok(mul) => {
                    if !self.mul_disabled {
                        self.result += mul.lhs * mul.rhs;
                    }
                    continue;
                }
                Err(()) => {}
            }

            self.next();
        }
        Ok(())
    }

    pub fn parse_dodont(&mut self) -> Result<(), ()> {
        if !self.feature_do_dont {
            return Err(());
        }
        self.savepoint();
        if self.parse_do().is_ok() {
            return Ok(());
        }
        self.savepoint();
        if self.parse_dont().is_ok() {
            return Ok(());
        }
        self.retreat();
        Err(())
    }

    pub fn parse_do(&mut self) -> Result<(), ()> {
        self.parse_literal("do()")?;
        self.mul_disabled = false;
        Ok(())
    }

    pub fn parse_dont(&mut self) -> Result<(), ()> {
        self.parse_literal("don't()")?;
        self.mul_disabled = true;
        Ok(())
    }

    pub fn parse_mul(&mut self) -> Result<Mul, ()> {
        self.savepoint();
        self.parse_literal("mul")?;
        self.parse_literal("(")?;
        let lhs = self.parse_number();
        if lhs.is_none() {
            self.retreat();
            return Err(());
        }
        self.parse_literal(",")?;
        let rhs = self.parse_number();
        if rhs.is_none() {
            self.retreat();
            return Err(());
        }
        self.parse_literal(")")?;
        Ok(Mul {
            lhs: lhs.unwrap(),
            rhs: rhs.unwrap(),
        })
    }

    pub fn parse_number(&mut self) -> Option<usize> {
        let mut num: Option<usize> = None;
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() {
                if let Some(n) = num {
                    num = Some(n * 10 + (b - b'0') as usize);
                } else {
                    num = Some((b - b'0') as usize);
                }
            } else {
                return num;
            }
            self.next();
        }
        num
    }

    pub fn parse_literal(&mut self, kw: &'static str) -> Result<(), ()> {
        self.savepoint();
        for (i, &b) in kw.as_bytes().iter().enumerate() {
            if self.peek() != Some(b) {
                self.retreat();
                return Err(());
            }
            self.next();
        }
        Ok(())
    }
}

pub fn day3_part1() -> usize {
    let (mmap, input) = aoc_mmap_day_input(3);
    let mut parser = Parser::new(mmap, input, false);
    parser.parse().unwrap();
    parser.result
}

pub fn day3_part2() -> usize {
    let (mmap, input) = aoc_mmap_day_input(3);
    let mut parser = Parser::new(mmap, input, true);
    parser.parse().unwrap();
    parser.result
}
