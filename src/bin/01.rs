use std::iter::Sum;

use itertools::{any, sorted, Itertools};

pub fn part_one(input: &str) -> Option<u32> {
    // iterate through each line,
    // sum as we go
    // store that sum in a variable (maxValue) that's not going out of scope
    // when we hit an empty line: check sum vs maxValue, if bigger, replace.
    // clear sum and keep going
    let lines = input.lines();
    let mut max_calories: usize = 0;
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            sum = 0;
        } else {
            let calories_on_line = line.parse::<usize>().unwrap();
            sum += calories_on_line;
        }
        if sum > max_calories {
            max_calories = sum
        }
    }
    Some(max_calories.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut max_calories: [usize; 3] = [0, 0, 0];
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            if any(max_calories, |i| sum > i) {
                max_calories.sort();
                max_calories[0] = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<usize>().unwrap();
    }
    if any(max_calories, |i| sum > i) {
        max_calories.sort();
        max_calories[0] = sum;
    }
    Some(max_calories.iter().sum::<usize>().try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
