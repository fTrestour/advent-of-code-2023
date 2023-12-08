use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> u32 {
    let mut input = input.lines();

    let mut instructions = input.next().unwrap().trim().chars().cycle();

    input.next();

    let mut map = HashMap::new();
    while let Some(line) = input.next() {
        let line = line.trim();
        let mut line = line.split(" = ");

        let current_place = line.next().unwrap();
        let mut places = line
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ");
        let left = places.next().unwrap();
        let right = places.next().unwrap();

        map.insert(current_place, (left, right));
    }

    let mut next_stops = map.keys().filter(|key| key.ends_with('A')).collect_vec();
    let mut steps_count = 0;

    while next_stops.iter().any(|place| !place.ends_with('Z')) {
        let instruction = instructions.next().unwrap();

        next_stops = next_stops
            .iter()
            .map(|place| {
                let (left, right) = map.get(*place).unwrap();

                let test = match instruction {
                    'L' => left,
                    'R' => right,
                    _ => panic!("Invalid instruction"),
                };
                test
            })
            .collect_vec();

        steps_count = steps_count + 1;
    }

    steps_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_case = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(solve(test_case), 6);
    }
}
