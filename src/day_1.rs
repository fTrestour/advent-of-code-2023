pub fn solve(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_case = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve(test_case), 142);
    }
}
