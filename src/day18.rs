use crate::util::aoc_read_day_lines;
use std::collections::VecDeque;

fn is_in_bounds(width: usize, y: isize, x: isize) -> bool {
    y >= 0 && y < width as isize && x >= 0 && x < width as isize
}

fn get_index(width: usize, y: usize, x: usize) -> usize {
    y * width + x
}

fn can_reach_end(grid: &[i32], width: usize, start: (usize, usize), end: (usize, usize)) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; width * width];

    queue.push_back(start);
    visited[get_index(width, start.0, start.1)] = true;

    while let Some((y, x)) = queue.pop_front() {
        if (y, x) == end {
            return true;
        }

        for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if is_in_bounds(width, new_y, new_x) {
                let new_y = new_y as usize;
                let new_x = new_x as usize;
                let idx = get_index(width, new_y, new_x);
                if grid[idx] != 1 && !visited[idx] {
                    queue.push_back((new_y, new_x));
                    visited[idx] = true;
                }
            }
        }
    }
    false
}

pub fn day18_part1(test_input: Option<(Vec<String>, usize, usize)>) -> usize {
    let (lines, maze_side_length, number_pairs_amount_to_take) =
        test_input.unwrap_or((aoc_read_day_lines(18), 71, 1024));
    let number_pairs = lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    let start = (0, 0);
    let end = (maze_side_length - 1, maze_side_length - 1);

    let mut grid = vec![0; maze_side_length * maze_side_length];
    for (x, y) in number_pairs.iter().take(number_pairs_amount_to_take) {
        grid[get_index(maze_side_length, *y, *x)] = 1;
    }
    grid[get_index(maze_side_length, start.0, start.1)] = 2;

    let mut visited = vec![false; maze_side_length * maze_side_length];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[get_index(maze_side_length, start.0, start.1)] = true;

    while let Some(((y, x), steps)) = queue.pop_front() {
        if (y, x) == end {
            return steps;
        }

        for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if is_in_bounds(maze_side_length, new_y, new_x) {
                let new_y = new_y as usize;
                let new_x = new_x as usize;
                let idx = get_index(maze_side_length, new_y, new_x);
                if grid[idx] != 1 && !visited[idx] {
                    queue.push_back(((new_y, new_x), steps + 1));
                    visited[idx] = true;
                }
            }
        }
    }

    usize::MAX
}

pub fn day18_part2(test_input: Option<(Vec<String>, usize, usize)>) -> (usize, usize) {
    let (lines, maze_side_length, starting_number_pairs_amount_to_take) =
        test_input.unwrap_or((aoc_read_day_lines(18), 71, 1024));
    let number_pairs = lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    let start = (0, 0);
    let end = (maze_side_length - 1, maze_side_length - 1);

    let mut left = starting_number_pairs_amount_to_take;
    let mut right = number_pairs.len();
    let mut result = (0, 0);
    let mut grid = vec![0; maze_side_length * maze_side_length];

    while left <= right {
        let mid = left + (right - left) / 2;

        // Reset grid
        grid.fill(0);

        // Add walls
        for (x, y) in number_pairs.iter().take(mid) {
            grid[get_index(maze_side_length, *y, *x)] = 1;
        }
        grid[get_index(maze_side_length, start.0, start.1)] = 2;

        if can_reach_end(&grid, maze_side_length, start, end) {
            left = mid + 1;
        } else {
            result = number_pairs[mid - 1];
            right = mid - 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1_example() {
        let ret = day18_part1(Some((
            EXAMPLE.lines().map(|line| line.to_string()).collect(),
            7,
            12,
        )));
        println!("day18_part1: {}", ret);
        assert_eq!(ret, 22);
    }

    #[test]
    fn test_part1_input() {
        let ret = day18_part1(None);
        println!("day18_part1: {}", ret);
        assert_eq!(ret, 276);
    }

    #[test]
    fn test_part2_example() {
        let ret = day18_part2(Some((
            EXAMPLE.lines().map(|line| line.to_string()).collect(),
            7,
            12,
        )));
        println!("day18_part2: {:?}", ret);
        assert_eq!(ret, (6, 1));
    }

    #[test]
    fn test_part2_input() {
        let ret = day18_part2(None);
        println!("day18_part2: {:?}", ret);
        assert_eq!(ret, (60, 37));
    }
}
