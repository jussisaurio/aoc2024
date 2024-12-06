use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::util::aoc_read_day_lines;

const WIDTH: usize = 130;
const HEIGHT: usize = 130;
const ARRAY_SIZE: usize = WIDTH * HEIGHT;
const VISITED_ARRAY_SIZE: usize = 4 * ARRAY_SIZE;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Terrain {
    #[default]
    Empty,
    Wall,
}

impl Terrain {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Terrain::Empty),
            '#' => Some(Terrain::Wall),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Guard {
    fn from_char_and_pos(c: char, x: usize, y: usize) -> Self {
        match c {
            '^' => Guard {
                direction: Direction::Up,
                x,
                y,
            },
            'v' => Guard {
                direction: Direction::Down,
                x,
                y,
            },
            '>' => Guard {
                direction: Direction::Right,
                x,
                y,
            },
            '<' => Guard {
                direction: Direction::Left,
                x,
                y,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct VisitedWhileFacingDirection {
    vec: [bool; VISITED_ARRAY_SIZE],
}

impl VisitedWhileFacingDirection {
    fn new() -> Self {
        Self {
            vec: [false; VISITED_ARRAY_SIZE],
        }
    }

    fn get(&self, x: usize, y: usize, direction: Direction) -> bool {
        self.vec[4 * (y * WIDTH + x) + direction as usize]
    }

    fn set(&mut self, x: usize, y: usize, direction: Direction, value: bool) {
        self.vec[4 * (y * WIDTH + x) + direction as usize] = value;
    }
}

#[derive(Debug, Clone, Copy)]
struct FlatVec<T> {
    vec: [T; ARRAY_SIZE],
}

impl<T: Copy + Default> FlatVec<T> {
    fn new() -> Self {
        Self {
            vec: [T::default(); ARRAY_SIZE],
        }
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[y * WIDTH + x]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        self.vec[y * WIDTH + x] = value;
    }
}

#[derive(Debug, Clone, Copy)]
struct Map {
    coords: FlatVec<Terrain>,
    visited: FlatVec<bool>,
    visited_while_facing_direction: VisitedWhileFacingDirection,
    visited_count: usize,
    guard: Guard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EndState {
    Exits,
    Loops,
}

impl Map {
    fn from_lines(lines: &[String]) -> Self {
        let mut coords = FlatVec::new();
        let mut visited = FlatVec::new();
        let mut visited_count = 0;
        let mut guard = None;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(terrain) = Terrain::from_char(c) {
                    coords.set(x, y, terrain);
                } else {
                    guard = Some(Guard::from_char_and_pos(c, x, y));
                    coords.set(x, y, Terrain::Empty);
                    visited.set(x, y, true);
                    visited_count += 1;
                }
            }
        }

        Map {
            coords,
            visited,
            visited_while_facing_direction: VisitedWhileFacingDirection::new(),
            visited_count,
            guard: guard.unwrap(),
        }
    }

    fn run(&mut self) {
        while self.move_guard() {
            if !self.visited.get(self.guard.x, self.guard.y) {
                self.visited.set(self.guard.x, self.guard.y, true);
                self.visited_count += 1;
            }
        }
    }

    fn run_until_exits_or_loops(&mut self) -> EndState {
        self.visited_while_facing_direction.set(
            self.guard.x,
            self.guard.y,
            self.guard.direction,
            true,
        );
        while self.move_guard() {
            if self.visited_while_facing_direction.get(
                self.guard.x,
                self.guard.y,
                self.guard.direction,
            ) {
                return EndState::Loops;
            }
            self.visited_while_facing_direction.set(
                self.guard.x,
                self.guard.y,
                self.guard.direction,
                true,
            );
        }
        EndState::Exits
    }

    fn move_guard(&mut self) -> bool {
        let is_exiting = match self.guard.direction {
            Direction::Up => self.guard.y == 0,
            Direction::Down => self.guard.y == HEIGHT - 1,
            Direction::Left => self.guard.x == 0,
            Direction::Right => self.guard.x == WIDTH - 1,
        };
        if is_exiting {
            return false;
        }

        let next_coord_candidate = match self.guard.direction {
            Direction::Up => (self.guard.x, self.guard.y - 1),
            Direction::Down => (self.guard.x, self.guard.y + 1),
            Direction::Left => (self.guard.x - 1, self.guard.y),
            Direction::Right => (self.guard.x + 1, self.guard.y),
        };

        match self
            .coords
            .get(next_coord_candidate.0, next_coord_candidate.1)
        {
            Terrain::Wall => {
                self.guard.direction = self.guard.direction.turn_right();
                return true;
            }
            Terrain::Empty => {}
        }

        let next_coord = match self.guard.direction {
            Direction::Up => (self.guard.x, self.guard.y - 1),
            Direction::Down => (self.guard.x, self.guard.y + 1),
            Direction::Left => (self.guard.x - 1, self.guard.y),
            Direction::Right => (self.guard.x + 1, self.guard.y),
        };

        self.guard.x = next_coord.0;
        self.guard.y = next_coord.1;
        true
    }
}

pub fn day6_part1() -> usize {
    let lines = aoc_read_day_lines(6);
    let mut map = Map::from_lines(&lines);
    map.run();
    map.visited_count
}

pub fn day6_part2() -> usize {
    let lines = aoc_read_day_lines(6);
    let map = Map::from_lines(&lines);

    let coords: Vec<(usize, usize)> = (0..HEIGHT)
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            *map.coords.get(x, y) != Terrain::Wall && (x, y) != (map.guard.x, map.guard.y)
        })
        .collect();

    coords
        .par_iter()
        .map(|&(x, y)| {
            let mut map_clone = map.clone();
            map_clone.coords.set(x, y, Terrain::Wall);
            map_clone.run_until_exits_or_loops() as usize
        })
        .sum()
}
