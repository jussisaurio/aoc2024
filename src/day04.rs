use std::collections::HashSet;

use crate::util::aoc_read_day_lines;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Letter::X,
            'M' => Letter::M,
            'A' => Letter::A,
            'S' => Letter::S,
            _ => unreachable!(),
        }
    }

    fn next(&self) -> Option<Letter> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
        }
    }
}

const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const UP: (isize, isize) = (-1, 0);
const RIGHT_UP: (isize, isize) = (-1, 1);
const RIGHT_DOWN: (isize, isize) = (1, 1);
const LEFT_UP: (isize, isize) = (-1, -1);
const LEFT_DOWN: (isize, isize) = (1, -1);

const DIRECTIONS: [(isize, isize); 8] = [
    RIGHT, DOWN, LEFT, UP, RIGHT_UP, LEFT_UP, RIGHT_DOWN, LEFT_DOWN,
];

pub fn day4_part1() -> usize {
    let lines = aoc_read_day_lines(4);
    let mut letters: Vec<Vec<Letter>> = Vec::new();
    let mut x_locations: Vec<(usize, usize)> = Vec::new();
    for y in 0..lines.len() {
        let mut row: Vec<Letter> = Vec::new();
        for x in 0..lines[y].len() {
            let letter = Letter::from_char(lines[y].chars().nth(x).unwrap());
            if letter == Letter::X {
                x_locations.push((x, y));
            }
            row.push(letter);
        }
        letters.push(row);
    }

    let mut ways_to_form_xmas = 0;
    for x_location in x_locations {
        for direction in DIRECTIONS.iter() {
            let mut current_letter = Letter::X;
            let mut current_location = x_location;

            loop {
                let new_location = (
                    current_location.0 as isize + direction.0,
                    current_location.1 as isize + direction.1,
                );
                if new_location.0 < 0
                    || new_location.0 >= letters[0].len() as isize
                    || new_location.1 < 0
                    || new_location.1 >= letters.len() as isize
                {
                    break;
                }
                let new_letter = letters[new_location.1 as usize][new_location.0 as usize];
                current_location = (new_location.0 as usize, new_location.1 as usize);
                if new_letter == current_letter.next().unwrap() {
                    if new_letter == Letter::S {
                        ways_to_form_xmas += 1;
                        break;
                    }
                    current_letter = new_letter;
                } else {
                    break;
                }
            }
        }
    }
    ways_to_form_xmas
}

const OPPOSITE_DIRECTION_QUADRANT: [(isize, isize); 4] = [LEFT_DOWN, RIGHT_UP, LEFT_UP, RIGHT_DOWN];

pub fn day4_part2() -> usize {
    let lines = aoc_read_day_lines(4);
    let mut letters: Vec<Vec<Letter>> = Vec::new();
    let mut a_locations: Vec<(usize, usize)> = Vec::new();
    for y in 0..lines.len() {
        let mut row: Vec<Letter> = Vec::new();
        for x in 0..lines[y].len() {
            let letter = Letter::from_char(lines[y].chars().nth(x).unwrap());
            if letter == Letter::A {
                a_locations.push((x, y));
            }
            row.push(letter);
        }
        letters.push(row);
    }

    let mut ways_to_form_xmas = 0;
    for a_location in a_locations {
        let letter_in_quadrant = OPPOSITE_DIRECTION_QUADRANT.iter().map(|dir| {
            let out_of_bounds = a_location.0 as isize + dir.0 < 0
                || a_location.0 as isize + dir.0 >= letters[0].len() as isize
                || a_location.1 as isize + dir.1 < 0
                || a_location.1 as isize + dir.1 >= letters.len() as isize;
            if out_of_bounds {
                None
            } else {
                Some(
                    letters[(a_location.1 as isize + dir.1) as usize]
                        [(a_location.0 as isize + dir.0) as usize],
                )
            }
        });

        let letter_in_quadrant: Vec<Option<Letter>> = letter_in_quadrant.collect();
        if letter_in_quadrant.iter().any(Option::is_none) {
            continue;
        }
        let letter_in_quadrant: Vec<Letter> =
            letter_in_quadrant.iter().filter_map(|l| *l).collect();
        // there needs to be exactly two M and two S, and 0-1 cant be the same, and 2-3 cant be the same
        if letter_in_quadrant.len() != 4
            || letter_in_quadrant
                .iter()
                .filter(|l| **l == Letter::M)
                .count()
                != 2
            || letter_in_quadrant
                .iter()
                .filter(|l| **l == Letter::S)
                .count()
                != 2
        {
            continue;
        }
        if letter_in_quadrant[0] == letter_in_quadrant[1]
            || letter_in_quadrant[2] == letter_in_quadrant[3]
        {
            continue;
        }
        ways_to_form_xmas += 1;
    }
    ways_to_form_xmas
}
