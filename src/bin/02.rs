use std::str::Split;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|f| f.split(' '))
            .map(compare_for_total)
            .map(|x| x as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|f| f.split(' '))
            .map(rigged_outcome)
            .map(|x| x as u32)
            .sum::<u32>(),
    )
}

fn compare_for_total(mut input: Split<char>) -> usize {
    let opponent = match input.next().unwrap() {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let player = match input.next().unwrap() {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!(),
    };
    let outcome_amount = get_result_of_round(opponent, &player);
    match player {
        RPS::Rock => outcome_amount + 1,
        RPS::Paper => outcome_amount + 2,
        RPS::Scissors => outcome_amount + 3,
    }
}

fn rigged_outcome(mut input: Split<char>) -> usize {
    let opponent = match input.next().unwrap() {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let (player, score) = match input.next().unwrap() {
        "X" => match opponent {
            RPS::Rock => (RPS::Scissors, 3),
            RPS::Paper => (RPS::Rock, 1),
            RPS::Scissors => (RPS::Paper, 2),
        },
        "Y" => match opponent {
            RPS::Rock => (RPS::Rock, 1),
            RPS::Paper => (RPS::Paper, 2),
            RPS::Scissors => (RPS::Scissors, 3),
        },
        "Z" => match opponent {
            RPS::Rock => (RPS::Paper, 2),
            RPS::Paper => (RPS::Scissors, 3),
            RPS::Scissors => (RPS::Rock, 1),
        },
        _ => panic!(),
    };
    get_result_of_round(opponent, &player) + score
}

fn get_result_of_round(opponent: RPS, player: &RPS) -> usize {
    return match player {
        RPS::Rock => match opponent {
            RPS::Rock => 3,
            RPS::Paper => 0,
            RPS::Scissors => 6,
        },
        RPS::Paper => match opponent {
            RPS::Rock => 6,
            RPS::Paper => 3,
            RPS::Scissors => 0,
        },
        RPS::Scissors => match opponent {
            RPS::Rock => 0,
            RPS::Paper => 6,
            RPS::Scissors => 3,
        },
    };
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
