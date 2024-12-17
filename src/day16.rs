use crate::util::aoc_read_day_lines;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn forward(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    y: usize,
    x: usize,
    direction: Direction,
    path: Option<Vec<(usize, usize, Direction)>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    state: State,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day16_part1(test_input: Option<String>) -> usize {
    day16(test_input, false)
}

pub fn day16_part2(test_input: Option<String>) -> usize {
    day16(test_input, true)
}

fn is_in_bounds(grid: &Vec<Vec<char>>, y: isize, x: isize) -> bool {
    y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize
}

pub fn day16(test_input: Option<String>, is_part2: bool) -> usize {
    let lines = if let Some(input) = test_input {
        input.lines().map(|s| s.to_string()).collect()
    } else {
        aoc_read_day_lines(16)
    };

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    let initial_state = State {
        y: start.0,
        x: start.1,
        direction: Direction::Right,
        path: if is_part2 {
            Some(vec![(start.0, start.1, Direction::Right)])
        } else {
            None
        },
    };
    heap.push(Node {
        state: initial_state.clone(),
        cost: 0,
    });
    distances.insert(
        (initial_state.y, initial_state.x, initial_state.direction),
        0,
    );

    let mut paths_to_end = vec![];
    let mut min_cost_seen = usize::MAX;

    while let Some(Node { state, cost }) = heap.pop() {
        if state.y == end.0 && state.x == end.1 {
            paths_to_end.push((cost, state.path.clone()));
            if cost < min_cost_seen {
                min_cost_seen = cost;
            }
            if !is_part2 {
                break;
            } else if cost > min_cost_seen {
                break;
            }
        }

        // Try moving forward
        let (dy, dx) = state.direction.forward();
        let new_row = state.y as isize + dy;
        let new_col = state.x as isize + dx;

        if is_in_bounds(&grid, new_row, new_col) {
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            if grid[new_row][new_col] != '#' {
                let mut new_path = state.path.clone();
                if let Some(path) = &mut new_path {
                    path.push((new_row, new_col, state.direction));
                }
                let new_state = State {
                    y: new_row,
                    x: new_col,
                    direction: state.direction,
                    path: new_path,
                };
                let new_cost = cost + 1;
                let key = (new_row, new_col, state.direction);
                if new_cost <= *distances.get(&key).unwrap_or(&usize::MAX) {
                    heap.push(Node {
                        state: new_state,
                        cost: new_cost,
                    });
                    distances.insert(key, new_cost);
                }
            }
        }

        // Try turning left
        let mut new_path = state.path.clone();
        let new_direction = state.direction.turn_left();
        if let Some(path) = &mut new_path {
            path.push((state.y, state.x, new_direction));
        }
        let new_state = State {
            y: state.y,
            x: state.x,
            direction: new_direction,
            path: new_path,
        };
        let new_cost = cost + 1000;
        let key = (state.y, state.x, new_direction);
        if new_cost <= *distances.get(&key).unwrap_or(&usize::MAX) {
            heap.push(Node {
                state: new_state,
                cost: new_cost,
            });
            distances.insert(key, new_cost);
        }

        // Try turning right
        let mut new_path = state.path.clone();
        let new_direction = state.direction.turn_right();
        if let Some(path) = &mut new_path {
            path.push((state.y, state.x, new_direction));
        }
        let new_state = State {
            y: state.y,
            x: state.x,
            direction: new_direction,
            path: new_path,
        };
        let new_cost = cost + 1000;
        let key = (state.y, state.x, new_direction);
        if new_cost <= *distances.get(&key).unwrap_or(&usize::MAX) {
            heap.push(Node {
                state: new_state,
                cost: new_cost,
            });
            distances.insert(key, new_cost);
        }
    }

    paths_to_end.sort_by(|a, b| a.0.cmp(&b.0));
    if !is_part2 {
        paths_to_end.first().unwrap().0
    } else {
        // take until cost increases
        let mut last_cost = usize::MAX;
        let mut shortest_paths = vec![];
        for (cost, path) in paths_to_end {
            if cost > last_cost {
                break;
            }
            last_cost = cost;
            shortest_paths.push(path);
        }
        let mut unique_coords = HashSet::new();
        for path in shortest_paths {
            for (row, col, _) in path.unwrap() {
                unique_coords.insert((row, col));
            }
        }
        unique_coords.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_day16_part1() {
        assert_eq!(day16_part1(Some(EXAMPLE.to_string())), 7036);
    }

    #[test]
    fn test_day16_part2() {
        assert_eq!(day16_part2(Some(EXAMPLE.to_string())), 45);
    }
}
