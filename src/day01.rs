use crate::util::aoc_read_day_lines;
use std::collections::HashMap;

pub fn day1_part1() -> i32 {
    let lines = aoc_read_day_lines(1);
    let lines_split: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    // e.g. [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    for line in lines_split {
        first_numbers.push(line[0]);
        second_numbers.push(line[1]);
    }
    first_numbers.sort();
    second_numbers.sort();

    let mut sum_of_abs_diffs = 0;
    for i in 0..first_numbers.len() {
        sum_of_abs_diffs += (first_numbers[i] - second_numbers[i]).abs();
    }
    sum_of_abs_diffs
}

pub fn day1_part2() -> i32 {
    let lines = aoc_read_day_lines(1);
    let lines_split: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    // e.g. [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    for line in lines_split {
        first_numbers.push(line[0]);
        second_numbers.push(line[1]);
    }

    let mut second_list_occurrence_map = HashMap::new();
    for number in second_numbers {
        *second_list_occurrence_map.entry(number).or_insert(0) += 1;
    }
    let mut sum = 0;
    for number in first_numbers {
        sum += number * *second_list_occurrence_map.get(&number).unwrap_or(&0);
    }
    sum
}
