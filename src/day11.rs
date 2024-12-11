use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::util::aoc_read_day_lines;

pub fn day11_part1() -> usize {
    let line = aoc_read_day_lines(11).pop().unwrap();
    let stones = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    stones
        .par_iter()
        .map(|stone| blinker(*stone, 25, &mut HashMap::new()))
        .sum()
}

pub fn day11_part2() -> usize {
    let line = aoc_read_day_lines(11).pop().unwrap();
    let stones = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    stones
        .par_iter()
        .map(|stone| blinker(*stone, 75, &mut HashMap::new()))
        .sum()
}

fn blinker(stone: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(ret) = cache.get(&(stone, blinks)) {
        return *ret;
    }
    if stone == 0 {
        let ret = blinker(1, blinks - 1, cache);
        cache.insert((stone, blinks), ret);
        return ret;
    }
    let num_digits = {
        let mut n = stone;
        let mut count = 0;
        while n > 0 {
            count += 1;
            n /= 10;
        }
        count
    };
    if num_digits % 2 == 0 {
        let (first_half, second_half) = split_int_in_two(stone, num_digits);
        let ret1 = blinker(first_half, blinks - 1, cache);
        let ret2 = blinker(second_half, blinks - 1, cache);
        let ret = ret1 + ret2;
        cache.insert((stone, blinks), ret);
        return ret;
    }

    let ret = blinker(stone * 2024, blinks - 1, cache);
    cache.insert((stone, blinks), ret);
    return ret;
}

fn split_int_in_two(num: usize, num_digits: usize) -> (usize, usize) {
    assert!(num_digits % 2 == 0);
    let half_digits = num_digits / 2;
    let divisor = 10_usize.pow(half_digits as u32);
    let second_half = num % divisor;
    let first_half = num / divisor;
    (first_half, second_half)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_int_in_two() {
        assert_eq!(split_int_in_two(10, 2), (1, 0));
        assert_eq!(split_int_in_two(99, 2), (9, 9));
        assert_eq!(split_int_in_two(1000, 4), (10, 0));
        assert_eq!(split_int_in_two(123456, 6), (123, 456));
        assert_eq!(split_int_in_two(1234567890, 10), (12345, 67890));
        assert_eq!(split_int_in_two(1234567890123456, 16), (12345678, 90123456));
        assert_eq!(
            split_int_in_two(123456789012345678, 18),
            (123456789, 12345678)
        );
        assert_eq!(
            split_int_in_two(12345678901234567890, 20),
            (1234567890, 1234567890)
        );
    }
}
