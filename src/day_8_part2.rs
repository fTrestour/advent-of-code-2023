use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    let mut input = input.lines();

    let instructions = input.next().unwrap().trim().chars().cycle();

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

    let starting_points = map.keys().filter(|key| key.ends_with('A')).collect_vec();

    for starting in starting_points {
        let mut instructions = instructions.clone();
        let mut seen_places = vec![];

        let mut next_place = *starting;
        loop {
            let (left, right) = map.get(next_place).unwrap();
            next_place = match instructions.next().unwrap() {
                'L' => left,
                'R' => right,
                _ => panic!("Invalid instruction"),
            };

            if next_place.ends_with('Z') && seen_places.contains(&next_place) {
                break;
            }
            seen_places.push(next_place);
        }

        let (init_len, _) = seen_places
            .iter()
            .find_position(|place| **place == next_place)
            .unwrap();
        let init_len = (init_len + 1) as u64;
        println!("{}", init_len);
    }

    // Small cheat but this program doesn't actually compute the answer
    // The actual answer is the least common multiplier of all init_len values
    todo!()
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
