pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line_digits = line
                .chars()
                .into_iter()
                .filter_map(|char| char.to_digit(10));

            let first_digit = line_digits.clone().next().unwrap();
            let last_digit = line_digits.last().unwrap();

            first_digit * 10 + last_digit
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut line = line.trim();
            dbg!(&line);

            let mut digits = vec![];

            let patterns = vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ];

            let mut matching_digit;
            loop {
                matching_digit = patterns
                    .iter()
                    .find(|pattern| line.strip_prefix(*pattern).is_some());

                if let Some(digit) = matching_digit {
                    let digit = match *digit {
                        "one" | "1" => 1,
                        "two" | "2" => 2,
                        "three" | "3" => 3,
                        "four" | "4" => 4,
                        "five" | "5" => 5,
                        "six" | "6" => 6,
                        "seven" | "7" => 7,
                        "eight" | "8" => 8,
                        "nine" | "9" => 9,
                        _ => panic!(),
                    };

                    digits.push(digit);
                }

                let (_, rest) = line.split_at(1);
                line = rest;

                if line.is_empty() {
                    break;
                }
            }

            dbg!(&digits);

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_case = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve_part1(test_case), 142);
    }

    #[test]
    fn test_solve_part2() {
        let test_case = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(solve_part2(test_case), 281);
    }
}
