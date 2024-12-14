use std::collections::HashSet;

use crate::util::aoc_read_day_lines;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

pub fn day14_part1(test_input: Option<(Vec<String>, usize, usize)>) -> usize {
    let (lines, width, height) =
        test_input.unwrap_or_else(|| (aoc_read_day_lines(14), WIDTH, HEIGHT));
    let mut lines = lines
        .iter()
        .map(|line| {
            let (p, v) = line[2..].split_once(" v=").unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            (
                (px.parse::<isize>().unwrap(), py.parse::<isize>().unwrap()),
                (vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    const ITERS: usize = 100;
    for (p, v) in lines.iter_mut() {
        p.0 = (p.0 + v.0 * ITERS as isize).rem_euclid(width as isize);
        p.1 = (p.1 + v.1 * ITERS as isize).rem_euclid(height as isize);
    }

    let mut in_quadrant_top_left = 0;
    let mut in_quadrant_top_right = 0;
    let mut in_quadrant_bottom_left = 0;
    let mut in_quadrant_bottom_right = 0;

    let mid_x: isize = width as isize / 2;
    let mid_y: isize = height as isize / 2;

    for (p, _) in lines.iter() {
        if p.0 == mid_x || p.1 == mid_y {
            continue;
        }
        if p.0 < mid_x && p.1 < mid_y {
            in_quadrant_top_left += 1;
        } else if p.0 > mid_x && p.1 < mid_y {
            in_quadrant_top_right += 1;
        } else if p.0 < mid_x && p.1 > mid_y {
            in_quadrant_bottom_left += 1;
        } else {
            in_quadrant_bottom_right += 1;
        }
    }

    in_quadrant_top_left
        * in_quadrant_top_right
        * in_quadrant_bottom_left
        * in_quadrant_bottom_right
}

pub fn day14_part2(test_input: Option<(Vec<String>, usize, usize)>) -> usize {
    let (lines, width, height) =
        test_input.unwrap_or_else(|| (aoc_read_day_lines(14), WIDTH, HEIGHT));
    let mut lines = lines
        .iter()
        .map(|line| {
            let (p, v) = line[2..].split_once(" v=").unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            (
                (px.parse::<isize>().unwrap(), py.parse::<isize>().unwrap()),
                (vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut points_set = lines.iter().map(|(p, _)| p).collect::<HashSet<_>>();
    let mut iter = 0;
    'outer: loop {
        for (p, v) in lines.iter_mut() {
            p.0 = (p.0 + v.0).rem_euclid(width as isize);
            p.1 = (p.1 + v.1).rem_euclid(height as isize);
        }
        points_set = lines.iter().map(|(p, _)| p).collect::<HashSet<_>>();
        iter += 1;
        // heuristic: christmas tree will have a long stretch of coordinates on a single x row
        // vertically
        for x in 0..WIDTH {
            let mut longest_span = 0;
            let mut count = 0;
            for y in 0..HEIGHT {
                if points_set.contains(&(x as isize, y as isize)) {
                    count += 1;
                } else {
                    if count > longest_span {
                        longest_span = count;
                    }
                    count = 0;
                }
            }
            if longest_span > 10 {
                println!("x: {}, count: {}", x, longest_span);
                // cls
                println!("\x1b[2J\x1b[1;1H");
                // print grid
                for y in 0..height {
                    let line_str = (0..width)
                        .map(|x| {
                            if points_set.contains(&(x as isize, y as isize)) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>();
                    println!("{}", line_str);
                }
                println!("iter: {}", iter);
                break 'outer;
            }
        }
    }
    iter
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_day14_part1() {
        assert_eq!(
            day14_part1(Some((
                EXAMPLE.lines().map(|s| s.to_string()).collect(),
                11,
                7
            ))),
            12
        );
    }

    #[test]
    fn test_euclid() {
        assert_eq!((10 as isize).rem_euclid(4), 2);
        assert_eq!((-5 as isize).rem_euclid(4), 3);
    }
}
