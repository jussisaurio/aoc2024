use std::collections::HashMap;

use crate::util::aoc_read_day_lines;

fn ways_to_make_towel(
    towel_patterns: &Vec<&str>,
    towel: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if towel.len() == 0 {
        return 1;
    }

    if memo.contains_key(towel) {
        return *memo.get(towel).unwrap();
    }

    let ways = towel_patterns
        .iter()
        .filter(|tp| towel.starts_with(**tp))
        .map(|tp| {
            let new_towel = towel.strip_prefix(*tp).unwrap();
            ways_to_make_towel(towel_patterns, new_towel, memo)
        })
        .sum();

    memo.insert(towel.to_string(), ways);
    ways
}

fn day19(input: Option<Vec<String>>, mapper: impl Fn(usize) -> usize) -> usize {
    let lines = input.unwrap_or(aoc_read_day_lines(19));
    let towel_patterns = lines[0].split(',').map(|s| s.trim()).collect::<Vec<_>>();
    let mut memo = HashMap::new();

    let desired_towels = lines
        .iter()
        .skip(2)
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    desired_towels
        .iter()
        .map(|towel| mapper(ways_to_make_towel(&towel_patterns, towel, &mut memo)))
        .sum()
}

pub fn day19_part1(input: Option<Vec<String>>) -> usize {
    day19(input, |ways| if ways > 0 { 1 } else { 0 })
}

pub fn day19_part2(input: Option<Vec<String>>) -> usize {
    day19(input, |ways| ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(
            day19_part1(Some(EXAMPLE.lines().map(|s| s.to_string()).collect())),
            6
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(day19_part1(None), 327);
    }

    #[test]
    fn test_part2_example() {
        const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(
            day19_part2(Some(EXAMPLE.lines().map(|s| s.to_string()).collect())),
            16
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(day19_part2(None), 772696486795255);
    }
}
