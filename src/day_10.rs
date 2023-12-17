use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::grid_input::{Direction, GridInput, Position};

pub fn solve_part1(input: &str) -> u32 {
    let input = parse(input);
    let main_loop = find_main_loop(&input);

    *main_loop.values().map(|(d, _)| d).max().unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    let grid = parse(input);
    let mut main_loop = find_main_loop(&grid)
        .into_iter()
        .sorted_by(|(a, _), (b, _)| {
            let y_comp = a.y.cmp(&b.y);
            if y_comp == Ordering::Equal {
                a.x.cmp(&b.x)
            } else {
                y_comp
            }
        });

    let mut inside = false;
    let mut count = 0;
    let mut last_x = main_loop.next().unwrap().0.x;

    for (position, (_, pipe)) in main_loop {
        if inside {
            count = count + position.x - (last_x + 1);
        }

        if pipe.is_vertical() {
            inside = !inside;
        }

        last_x = position.x;
    }

    count as u32
}

fn find_main_loop(input: &GridInput<Pipe>) -> HashMap<Position, (u32, Pipe)> {
    let (starting_position, _) = input
        .find_position(|pipe| *pipe == &Pipe::StartingPoint)
        .unwrap();

    let mut queue = vec![starting_position.clone()];
    let mut main_loop = HashMap::new();
    main_loop.insert(
        starting_position.clone(),
        (0, input.get(&starting_position).unwrap().unwrap().clone()),
    );

    while let Some(current_position) = queue.pop() {
        let (current_distance, current_pipe) = main_loop.get(&current_position).unwrap().clone();
        let new_distance = current_distance + 1;

        let adjacent_directions = get_adjacent_directions(&input, &current_position);
        let adjacent_directions = adjacent_directions
            .iter()
            .filter(|direction| current_pipe.output_directions().contains(direction))
            .collect_vec();
        for direction in adjacent_directions {
            let new_position = direction.to_position(&current_position).unwrap();

            if main_loop
                .get(&new_position)
                .map(|(d, _)| d < &new_distance)
                .unwrap_or(false)
            {
                continue;
            }

            let pipe = input.get(&new_position).unwrap().unwrap();
            let input_directions = &pipe.input_directions();
            if input_directions.contains(&direction) {
                if !queue.contains(&new_position) {
                    queue.push(new_position.clone());
                }
                main_loop.insert(new_position.clone(), (new_distance.clone(), pipe.clone()));
            }
        }
    }

    main_loop
}

fn parse(input: &str) -> GridInput<Pipe> {
    let input = input.replace(' ', "");
    let (line_len, _) = input.chars().find_position(|c| *c == '\n').unwrap();
    let max_position = Position::from_coordinates(line_len - 1, input.lines().count() - 1);

    let input = input
        .chars()
        .filter_map(|c| Pipe::try_from(c).ok())
        .collect_vec();
    GridInput {
        max_position,
        data: input,
    }
}

fn get_adjacent_directions(grid: &GridInput<Pipe>, p: &Position) -> Vec<Direction> {
    let mut around = vec![];
    if p.x != 0 {
        around.push(Direction::Left);
    }

    if p.x != grid.max_position.x {
        around.push(Direction::Right);
    }

    if p.y != 0 {
        around.push(Direction::Top);
    }

    if p.y != grid.max_position.y {
        around.push(Direction::Bottom);
    }

    around
}

#[derive(PartialEq, Debug, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPoint,
}

impl Pipe {
    pub fn input_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::Top, Direction::Bottom],
            Pipe::Horizontal => vec![Direction::Left, Direction::Right],
            Pipe::NorthEast => vec![Direction::Bottom, Direction::Left],
            Pipe::NorthWest => vec![Direction::Bottom, Direction::Right],
            Pipe::SouthWest => vec![Direction::Top, Direction::Right],
            Pipe::SouthEast => vec![Direction::Top, Direction::Left],
            Pipe::Ground => vec![],
            Pipe::StartingPoint => vec![],
        }
    }

    pub fn output_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::Top, Direction::Bottom],
            Pipe::Horizontal => vec![Direction::Left, Direction::Right],
            Pipe::NorthEast => vec![Direction::Top, Direction::Right],
            Pipe::NorthWest => vec![Direction::Top, Direction::Left],
            Pipe::SouthWest => vec![Direction::Bottom, Direction::Left],
            Pipe::SouthEast => vec![Direction::Bottom, Direction::Right],
            Pipe::Ground => vec![],
            Pipe::StartingPoint => vec![
                Direction::Bottom,
                Direction::Right,
                Direction::Top,
                Direction::Left,
            ],
        }
    }

    pub fn is_vertical(&self) -> bool {
        match *self {
            Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Pipe {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::StartingPoint),
            _ => Err("Invalid input".to_owned()),
        }
    }

    type Error = String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_simple_loop() {
        let test_case = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        assert_eq!(solve_part1(test_case), 4);
    }

    #[test]
    fn test_solve_part1_complex_loop() {
        let test_case = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";
        assert_eq!(solve_part1(test_case), 8);
    }

    #[test]
    fn test_solve_part2_simple_loop() {
        let test_case = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        assert_eq!(solve_part2(test_case), 4);
    }

    #[test]
    fn test_solve_part2_complex_loop() {
        let test_case = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        assert_eq!(solve_part2(test_case), 8);
    }

    #[test]
    fn test_solve_part2_more_loops() {
        let test_case = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve_part2(test_case), 10);
    }
}
