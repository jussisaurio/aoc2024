use crate::util::*;
use std::collections::VecDeque;

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(grid: &Vec<Vec<u8>>, start: (isize, isize), is_part_2: bool) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut result = 0;

    while let Some(((current_y, current_x), current_value)) = queue.pop_front() {
        if visited[current_y as usize][current_x as usize] {
            continue;
        }
        visited[current_y as usize][current_x as usize] = true;
        result += (current_value == 9) as usize;

        for (dx, dy) in CARDINAL_DIRECTIONS {
            let nx = current_x + dx;
            let ny = current_y + dy;
            let need_to_visit = ny >= 0
                && ny < grid.len() as isize
                && nx >= 0
                && nx < grid[0].len() as isize
                && !visited[ny as usize][nx as usize]
                && grid[ny as usize][nx as usize] == current_value + 1;
            if need_to_visit {
                queue.push_back(((ny, nx), current_value + 1));
            }
        }

        if is_part_2 {
            visited[current_y as usize][current_x as usize] = false;
        }
    }
    result
}

pub fn day10(is_part_2: bool) -> usize {
    let input = aoc_read_day_lines(10)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut starting_points = vec![];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 0 {
                starting_points.push((y as isize, x as isize));
            }
        }
    }

    starting_points
        .iter()
        .map(|start| bfs(&input, *start, is_part_2))
        .sum()
}

pub fn day10_part1() -> usize {
    day10(false)
}

pub fn day10_part2() -> usize {
    day10(true)
}
