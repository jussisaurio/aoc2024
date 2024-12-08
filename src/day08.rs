use crate::util::*;
use smallvec::{smallvec, SmallVec};

const SIDE_LENGTH: usize = 50;

fn is_in_bounds((x, y): (isize, isize)) -> bool {
    x < SIDE_LENGTH as isize && y < SIDE_LENGTH as isize && x >= 0 && y >= 0
}

pub fn day8(min_k: usize, max_k: usize) -> usize {
    let bytes = aoc_read_day_bytes(8);
    // 'z' ascii code is 122, '0' ascii code is 48
    let mut nodes: [SmallVec<[usize; 8]>; 122 - 48 + 1] = std::array::from_fn(|_| smallvec![]);
    let mut i = 0;
    for byte in bytes {
        if (byte as usize) < 48 {
            i += (byte == b'.') as usize;
            continue;
        }
        nodes[byte as usize - 48].push(i);
        i += 1;
    }
    let mut antinodes: [bool; SIDE_LENGTH * SIDE_LENGTH] = [false; SIDE_LENGTH * SIDE_LENGTH];
    let mut antinodes_count = 0;
    for positions in nodes.iter().filter(|positions| !positions.is_empty()) {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let x1 = positions[i] % SIDE_LENGTH;
                let y1 = positions[i] / SIDE_LENGTH;
                let x2 = positions[j] % SIDE_LENGTH;
                let y2 = positions[j] / SIDE_LENGTH;
                let xdiff = x2 as isize - x1 as isize;
                let ydiff = y2 as isize - y1 as isize;
                for k in min_k..=max_k {
                    let antinode1 = (
                        x1 as isize - xdiff * k as isize,
                        y1 as isize - ydiff * k as isize,
                    );
                    let antinode2 = (
                        x2 as isize + xdiff * k as isize,
                        y2 as isize + ydiff * k as isize,
                    );
                    let first_in_bounds = is_in_bounds(antinode1);
                    let second_in_bounds = is_in_bounds(antinode2);
                    if !first_in_bounds && !second_in_bounds {
                        break;
                    }
                    if first_in_bounds {
                        antinodes_count += !antinodes
                            [antinode1.0 as usize + antinode1.1 as usize * SIDE_LENGTH]
                            as usize;
                        antinodes[antinode1.0 as usize + antinode1.1 as usize * SIDE_LENGTH] = true;
                    }
                    if second_in_bounds {
                        antinodes_count += !antinodes
                            [antinode2.0 as usize + antinode2.1 as usize * SIDE_LENGTH]
                            as usize;
                        antinodes[antinode2.0 as usize + antinode2.1 as usize * SIDE_LENGTH] = true;
                    }
                }
            }
        }
    }
    antinodes_count
}

pub fn day8_part1() -> usize {
    day8(1, 1)
}

pub fn day8_part2() -> usize {
    day8(0, usize::MAX)
}
