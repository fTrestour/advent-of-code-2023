use itertools::Itertools;

pub fn solve_part1(input: &str) -> i32 {
    let mut results = vec![];

    for line in input.lines() {
        let history = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap());

        let prediction = predict_forward(history);

        results.push(prediction);
    }

    results.iter().sum()
}

pub fn solve_part2(input: &str) -> i32 {
    let mut results = vec![];

    for line in input.lines() {
        let history = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap());

        let prediction = predict_backward(history);

        results.push(prediction);
    }

    results.iter().sum()
}

fn predict_forward(mut history: impl Iterator<Item = i32>) -> i32 {
    let mut last_value = history.next().unwrap();
    let mut differences = vec![];

    for value in history {
        let difference = value - last_value;
        differences.push(difference);
        last_value = value;
    }
    if differences.iter().dedup().count() == 1 {
        last_value + differences.get(0).unwrap()
    } else {
        last_value + predict_forward(differences.into_iter())
    }
}

fn predict_backward(history: impl DoubleEndedIterator<Item = i32>) -> i32 {
    -predict_forward(history.rev().map(|value| -value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_case = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(solve_part1(test_case), 114);
    }

    #[test]
    fn test_solve_part2() {
        let test_case = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(solve_part2(test_case), 2);
    }
}
