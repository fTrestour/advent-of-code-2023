mod block;
mod crucible;

use std::{collections::VecDeque, u32};

use itertools::Itertools;

use crate::{
    day_17::{block::Block, crucible::Crucible},
    grid_input::{GridInput, Position},
};

pub fn solve_part1(input: &str) -> usize {
    let max_position = Position::from_coordinates(
        input.lines().next().unwrap().trim().len() - 1,
        input.lines().count() - 1,
    );
    let grid = GridInput {
        max_position,
        data: input
            .lines()
            .flat_map(|line| line.trim().chars())
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .map(Block::from)
            .collect_vec(),
    };
    println!("{}", grid);

    let crucible = Crucible::from(grid);
    let mut results = vec![];
    let mut queue = VecDeque::from(vec![crucible]);
    while let Some(crucible) = queue.pop_front() {
        let available_directions = crucible.available_directions();
        for direction in available_directions {
            let mut new_crucible = crucible.clone();
            new_crucible.r#move(&direction);

            if new_crucible.distance() == 0 {
                println!("{}", new_crucible);
                results.push(new_crucible);
            } else {
                let min_crucible = queue
                    .iter()
                    .filter(|crucible| crucible.position == new_crucible.position)
                    .min_by_key(|crucible| crucible.heat_loss);

                if let Some(min_crucible) = min_crucible {
                    if crucible.heat_loss < min_crucible.heat_loss {
                        queue.push_front(new_crucible);
                    }
                } else {
                    queue.push_front(new_crucible);
                }
            }

            let current_min = results.iter().map(|crucible| crucible.heat_loss).min();
            queue = VecDeque::from(
                queue
                    .iter()
                    .filter(|crucible| {
                        current_min
                            .map(|current_min| crucible.heat_loss < current_min)
                            .unwrap_or(true)
                    })
                    .sorted_by_key(|crucible| crucible.heat_loss + crucible.distance())
                    .map(|c| c.to_owned())
                    .collect_vec(),
            );
        }
    }

    results
        .iter()
        .map(|crucible| crucible.heat_loss)
        .min()
        .unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_case = "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";
        assert_eq!(solve_part1(test_case), 102);
    }
}
