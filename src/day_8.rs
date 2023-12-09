use std::collections::HashMap;

pub fn solve_part1(input: &str) -> u32 {
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

    dbg!(&map);

    let mut next_stop = "AAA";
    let mut steps_count = 0;

    while next_stop != "ZZZ" {
        let (left, right) = map.get(next_stop).unwrap();
        next_stop = match instructions.next().unwrap() {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid instruction"),
        };
        steps_count = steps_count + 1;

        dbg!(&next_stop, &steps_count);
    }

    steps_count

    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_case = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve_part1(test_case), 2);
    }

    #[test]
    fn test_solve_part1_6() {
        let test_case = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve_part1(test_case), 6);
    }
}
