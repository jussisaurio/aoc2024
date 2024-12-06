use crate::util::aoc_mmap_day_input;
use memmap::Mmap;
pub struct Parser {
    mmap: Mmap,
    input: &'static [u8],
    offset: usize,
    dampener: bool,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

impl Parser {
    pub fn new((mmap, input): (Mmap, &'static [u8]), dampener: bool) -> Self {
        Self {
            mmap,
            input,
            offset: 0,
            dampener,
        }
    }

    fn next(&mut self) {
        self.offset += 1;
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.offset).copied()
    }

    fn consume_until_next_newline_inclusive(&mut self) {
        while let Some(c) = self.peek() {
            if c == b'\n' {
                self.next();
                return;
            }
            self.next();
        }
    }

    fn check_sequence(&self, prev: u32, cur: u32, direction: Option<&Direction>) -> bool {
        // not ascending or descending
        if prev == cur {
            return false;
        }
        // too big of a jump
        if (prev as i32 - cur as i32).abs() > 3 {
            return false;
        }
        if let Some(dir) = direction {
            // changes direction
            if prev >= cur && *dir != Direction::Descending {
                return false;
            }
            if prev < cur && *dir != Direction::Ascending {
                return false;
            }
        }
        true
    }

    pub fn parse_line(&mut self, without: Option<usize>) -> Result<bool, ()> {
        let starting_offset = self.offset;
        let mut prev_num = None;
        let mut cur_num = None;
        let mut cur_num_idx = 0;
        let mut direction = None;
        let mut num_count = 0;

        while let Some(c) = self.peek() {
            if c == b'\n' {
                match (prev_num, cur_num) {
                    (Some(prev), Some(cur)) => {
                        self.next();
                        if !self.check_sequence(prev, cur, direction.as_ref()) {
                            if without.is_none() && self.dampener {
                                return Ok(true);
                            }
                            return Ok(false);
                        }
                        return Ok(true);
                    }
                    other => unreachable!("{:?}", other),
                }
            }
            if c.is_ascii_digit() {
                if let Some(prev) = cur_num {
                    cur_num = Some(prev * 10 + (c - b'0') as u32);
                } else {
                    cur_num = Some((c - b'0') as u32);
                }
            } else {
                assert!(c == b' ');
                if let Some(without) = without {
                    if cur_num_idx == without {
                        cur_num = None;
                        cur_num_idx += 1;
                        self.next();
                        continue;
                    }
                }
                if let Some(cur) = cur_num {
                    num_count += 1;
                }
                match (prev_num, cur_num) {
                    (Some(prev), Some(cur)) => {
                        if !self.check_sequence(prev, cur, direction.as_ref()) {
                            if without.is_none() && self.dampener {
                                for i in 0..num_count {
                                    self.offset = starting_offset;
                                    let valid = self.parse_line(Some(i))?;
                                    if valid {
                                        return Ok(true);
                                    }
                                }
                                return Ok(false);
                            }
                            self.consume_until_next_newline_inclusive();
                            return Ok(false);
                        }
                        if direction.is_none() {
                            direction = Some(if prev >= cur {
                                Direction::Descending
                            } else {
                                Direction::Ascending
                            });
                        }
                    }
                    _ => (),
                }
                prev_num = cur_num;
                cur_num = None;
                cur_num_idx += 1;
            }
            self.next();
        }
        match (prev_num, cur_num) {
            (Some(prev), Some(cur)) => {
                if !self.check_sequence(prev, cur, direction.as_ref()) {
                    if without.is_none() && self.dampener {
                        return Ok(true);
                    }
                    return Ok(false);
                }
                Ok(true)
            }
            _ => Err(()),
        }
    }

    pub fn parse(&mut self) -> Result<usize, ()> {
        let mut count = 0;
        while let Ok(valid) = self.parse_line(None) {
            assert!(self.peek() != Some(b'\n'));
            if valid {
                count += 1;
            }
        }
        Ok(count)
    }
}

fn day2(dampener: bool) -> Parser {
    Parser::new(aoc_mmap_day_input(2), dampener)
}

pub fn day2_part1() -> usize {
    day2(false).parse().unwrap()
}

pub fn day2_part2() -> usize {
    day2(true).parse().unwrap()
}
