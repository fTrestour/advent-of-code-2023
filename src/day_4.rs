use std::collections::{hash_map::RandomState, HashSet};

pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(solve_card).sum()
}

fn solve_card(line: &str) -> u32 {
    let mut line = line.trim().split(':').last().unwrap().split('|');

    let winning_numbers = line
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());
    let winning_numbers: HashSet<u32, RandomState> = HashSet::from_iter(winning_numbers);

    let numbers_i_have = line
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());

    let matches_number = numbers_i_have
        .filter(|n| winning_numbers.contains(n))
        .count() as u32;

    if matches_number == 0 {
        0
    } else {
        2_u32.pow(matches_number - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part1(test_case), 13);
    }
}
