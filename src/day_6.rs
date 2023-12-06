use itertools::Itertools;

pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace();

    let distances = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace();

    let races = distances.zip_eq(times).map(|(distance, time)| Race {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    });

    races
        .filter_map(|race| solve(race))
        .reduce(|acc, n| acc * n)
        .unwrap()
}

pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let race = Race { time, distance };
    solve(race).unwrap()
}

fn solve(race: Race) -> Option<usize> {
    // We want:
    // speed * remaining time > distance
    // time pressed * (time - time pressed)  > distance

    // With x = time pressed:
    // -x2 + time*x - distance > 0

    // Roots for the polynomial are:
    // (time +- sqrt(time^2 - 4*distance))/2
    let root1 =
        (race.time as f64 - ((race.time as f64).powi(2) - 4.0 * race.distance as f64).sqrt()) / 2.0;
    let root2 =
        (race.time as f64 + ((race.time as f64).powi(2) - 4.0 * race.distance as f64).sqrt()) / 2.0;

    // All integers in between these roots are valid answers
    let range = root1.floor() as u64..root2.ceil() as u64;
    range.try_len().map(|n| n - 1).ok()
}

struct Race {
    time: u64,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE: &str = "Time:      7  15   30
        Distance:  9  40  200";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_CASE), 288);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_CASE), 71503);
    }
}
