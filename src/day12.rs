use std::collections::HashSet;

use crate::util::aoc_read_day_lines;

struct Land {
    grid: Vec<Vec<char>>,
    explored: Vec<Vec<bool>>,
}

pub fn day12_part1(test_input: Option<Vec<String>>) -> usize {
    let grid = test_input
        .unwrap_or(aoc_read_day_lines(12))
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();
    let mut land = Land {
        grid,
        explored: vec![vec![false; width]; height],
    };
    for y in 0..height {
        for x in 0..width {
            let (area, perimeter) = part1_get_area_and_perimeter(&mut land, x, y).unwrap_or((0, 0));
            sum += perimeter * area;
        }
    }
    sum
}

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1_get_area_and_perimeter(land: &mut Land, x: usize, y: usize) -> Option<(usize, usize)> {
    if land.explored[y][x] {
        return None;
    }
    let plot_type = land.grid[y][x];
    let mut visited = vec![vec![false; land.grid[0].len()]; land.grid.len()];

    Some(part1_dfs(land, x, y, &mut visited, plot_type))
}

fn part1_dfs(
    land: &mut Land,
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
    plot_type: char,
) -> (usize, usize) {
    visited[y][x] = true;
    land.explored[y][x] = true;

    let mut area = 1;
    let mut perimeter = 0;

    for (dx, dy) in CARDINAL_DIRECTIONS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 || nx >= land.grid[0].len() as isize || ny >= land.grid.len() as isize {
            perimeter += 1;
            continue;
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if land.grid[ny][nx] != plot_type {
            perimeter += 1;
            continue;
        }

        if visited[ny][nx] {
            continue;
        }

        let (sub_area, sub_perimeter) = part1_dfs(land, nx, ny, visited, plot_type);
        area += sub_area;
        perimeter += sub_perimeter;
    }

    (area, perimeter)
}

pub fn day12_part2(test_input: Option<Vec<String>>) -> usize {
    let grid = test_input
        .unwrap_or(aoc_read_day_lines(12))
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();
    let mut land = Land {
        grid,
        explored: vec![vec![false; width]; height],
    };
    for y in 0..height {
        for x in 0..width {
            let (area, sides) = part2_get_area_and_sides(&mut land, x, y).unwrap_or((0, 0));
            sum += sides * area;
        }
    }
    sum
}

fn count_area_and_sides(land: &mut Land, start_x: usize, start_y: usize) -> (usize, usize) {
    let plot_type = land.grid[start_y][start_x];
    let mut visited = HashSet::new();
    let mut area = 0;
    let mut stack = vec![(start_x, start_y)];

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    // First pass: flood fill to count area and find edge
    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        land.explored[y][x] = true;
        area += 1;

        for (dx, dy) in CARDINAL_DIRECTIONS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0
                || ny < 0
                || nx >= land.grid[0].len() as isize
                || ny >= land.grid.len() as isize
            {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if is_same_plot_type(land, nx as isize, ny as isize, plot_type)
                && !visited.contains(&(nx, ny))
            {
                stack.push((nx, ny));
            }
        }
    }

    let wall_is_on_top = |x: usize, y: usize| {
        if y == 0 {
            return true;
        }
        if !is_same_plot_type(land, x as isize, y as isize - 1, plot_type) {
            return true;
        }
        false
    };
    let wall_is_on_bottom = |x: usize, y: usize| {
        if y == land.grid.len() - 1 {
            return true;
        }
        if !is_same_plot_type(land, x as isize, y as isize + 1, plot_type) {
            return true;
        }
        false
    };
    let wall_is_on_left = |x: usize, y: usize| {
        if x == 0 {
            return true;
        }
        if !is_same_plot_type(land, x as isize - 1, y as isize, plot_type) {
            return true;
        }
        false
    };
    let wall_is_on_right = |x: usize, y: usize| {
        if x == land.grid[0].len() - 1 {
            return true;
        }
        if !is_same_plot_type(land, x as isize + 1, y as isize, plot_type) {
            return true;
        }
        false
    };

    // sides are all unique combinations of:
    // wall on top, y coordinate in range of region (deduplicate contiguous x coordinates)
    // wall on bottom, y coordinate in range of region (deduplicate contiguous x coordinates)
    // wall on left, x coordinate in range of region (deduplicate contiguous y coordinates)
    // wall on right, x coordinate in range of region (deduplicate contiguous y coordinates)
    let mut sides = 0;
    for y in min_y..=max_y {
        let mut cur_side = false;
        for x in min_x..=max_x {
            if !visited.contains(&(x, y)) {
                cur_side = false;
                continue;
            }
            if wall_is_on_top(x, y) {
                if !cur_side {
                    sides += 1;
                }
                cur_side = true;
            } else {
                cur_side = false;
            }
        }
    }
    for y in min_y..=max_y {
        let mut cur_side = false;
        for x in min_x..=max_x {
            if !visited.contains(&(x, y)) {
                cur_side = false;
                continue;
            }
            if wall_is_on_bottom(x, y) {
                if !cur_side {
                    sides += 1;
                }
                cur_side = true;
            } else {
                cur_side = false;
            }
        }
    }
    for x in min_x..=max_x {
        let mut cur_side = false;
        for y in min_y..=max_y {
            if !visited.contains(&(x, y)) {
                cur_side = false;
                continue;
            }
            if wall_is_on_left(x, y) {
                if !cur_side {
                    sides += 1;
                }
                cur_side = true;
            } else {
                cur_side = false;
            }
        }
    }
    for x in min_x..=max_x {
        let mut cur_side = false;
        for y in min_y..=max_y {
            if !visited.contains(&(x, y)) {
                cur_side = false;
                continue;
            }
            if wall_is_on_right(x, y) {
                if !cur_side {
                    sides += 1;
                }
                cur_side = true;
            } else {
                cur_side = false;
            }
        }
    }

    (area, sides)
}

fn is_same_plot_type(land: &Land, x: isize, y: isize, cur_plot_type: char) -> bool {
    land.grid[y as usize][x as usize] == cur_plot_type
}

fn part2_get_area_and_sides(land: &mut Land, x: usize, y: usize) -> Option<(usize, usize)> {
    if land.explored[y][x] {
        return None;
    }
    let (area, sides) = count_area_and_sides(land, x, y);
    land.explored[y][x] = true;
    Some((area, sides))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn test_day12_part1() {
        assert_eq!(
            day12_part1(Some(EXAMPLE.lines().map(|line| line.to_string()).collect())),
            1930
        );
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(
            day12_part2(Some(EXAMPLE.lines().map(|line| line.to_string()).collect())),
            1206
        );
    }
}
