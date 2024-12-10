use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::util::*;
use std::collections::VecDeque;

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(grid: &Vec<u8>, start: isize, width: usize, height: usize, is_part_2: bool) -> usize {
    let mut visited = vec![false; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut result = 0;

    while let Some((current, current_value)) = queue.pop_front() {
        if visited[current as usize] {
            continue;
        }
        visited[current as usize] = true;
        result += (current_value == 9) as usize;

        let current_x = current % (width as isize);
        let current_y = current / (width as isize);

        for (dx, dy) in CARDINAL_DIRECTIONS {
            let nx = current_x + dx;
            let ny = current_y + dy;
            let n_idx = ny * (width as isize) + nx;
            let need_to_visit = ny >= 0
                && ny < height as isize
                && nx >= 0
                && nx < width as isize
                && !visited[n_idx as usize]
                && grid[n_idx as usize] == current_value + 1;
            if need_to_visit {
                queue.push_back((n_idx, current_value + 1));
            }
        }

        if is_part_2 {
            visited[current as usize] = false;
        }
    }
    result
}

pub fn day10(is_part_2: bool) -> usize {
    let mut height = 1;
    let mut width = 0;
    let input = aoc_read_day_bytes(10)
        .iter()
        .filter_map(|byte| {
            width += (height == 1 && *byte != b'\n') as usize;
            height += (*byte == b'\n') as usize;
            if *byte == b'\n' {
                return None;
            }
            Some(byte - b'0')
        })
        .collect::<Vec<u8>>();

    input
        .par_iter()
        .enumerate()
        .filter(|(_, &byte)| byte == 0)
        .map(|(start, _)| bfs(&input, start as isize, width, height, is_part_2))
        .sum()
}

pub fn day10_part1() -> usize {
    day10(false)
}

pub fn day10_part2() -> usize {
    day10(true)
}
