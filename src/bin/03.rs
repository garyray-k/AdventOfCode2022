use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split_line)
            .map(find_common_letter)
            .map(total_letters)
            .sum::<u32>(),
    )
}

fn split_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_common_letter(input: (&str, &str)) -> char {
    let first: HashSet<char> = input.0.chars().collect();
    input
        .1
        .chars()
        .filter(|x| first.contains(x))
        .find_or_first(|_x| true)
        .unwrap()
}

fn total_letters(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u8 + 1 - 'a' as u8) as u32,
        'A'..='Z' => (letter as u8 + 1 - 'A' as u8 + 26) as u32,
        _ => panic!(),
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
