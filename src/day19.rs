use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::util::aoc_read_day_lines;

fn ways_to_make_towel<'a>(towel_patterns: &'a Vec<&str>, towel: &'a str) -> usize {
    let mut dp = vec![0; towel.len() + 1];
    dp[0] = 1;
    for i in 0..towel.len() {
        for &pattern in towel_patterns {
            if towel[i..].starts_with(pattern) {
                dp[i + pattern.len()] += dp[i];
            }
        }
    }
    dp[towel.len()]
}

fn day19(input: Option<Vec<String>>, mapper: impl Fn(usize) -> usize + Send + Sync) -> usize {
    let lines = input.unwrap_or(aoc_read_day_lines(19));
    let towel_patterns = lines[0].split(',').map(|s| s.trim()).collect::<Vec<_>>();

    let desired_towels = lines
        .iter()
        .skip(2)
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    desired_towels
        .par_iter()
        .map(|towel| {
            let feasible_patterns = towel_patterns
                .iter()
                .filter(|tp| towel.contains(**tp))
                .cloned()
                .collect::<Vec<_>>();
            mapper(ways_to_make_towel(&feasible_patterns, towel))
        })
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
    fn test_part2_example2() {
        const EXAMPLE: &str = "a,ab,b,bc,c

abc";
        assert_eq!(
            day19_part2(Some(EXAMPLE.lines().map(|s| s.to_string()).collect())),
            3
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(day19_part2(None), 772696486795255);
    }
}
