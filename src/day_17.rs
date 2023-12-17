mod block;
mod crucible;

use std::{
    collections::{HashSet, VecDeque},
    u32,
};

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

    let crucible = Crucible::from(grid.clone());

    let mut visited = HashSet::new();
    let mut fronteer = HashSet::new();
    fronteer.insert(crucible.clone());

    loop {
        let min_on_fronteer = fronteer
            .iter()
            .min_by_key(|crucible| crucible.heat_loss)
            .unwrap()
            .to_owned();
        visited.insert(min_on_fronteer.position);

        if min_on_fronteer.position == grid.max_position {
            println!("{}", min_on_fronteer);
            return min_on_fronteer.heat_loss;
        }

        fronteer.remove(&min_on_fronteer);
        for direction in min_on_fronteer.available_directions() {
            let mut new_crucible = min_on_fronteer.clone();
            new_crucible.r#move(&direction);

            let previous = fronteer
                .iter()
                .find(|crucible| crucible.position == new_crucible.position);

            if let Some(previous) = previous {
                if new_crucible.heat_loss < previous.heat_loss {
                    fronteer.remove(&previous.clone());
                    fronteer.insert(new_crucible);
                }
            } else {
                fronteer.insert(new_crucible);
            }
        }

        println!("{}", min_on_fronteer);
    }
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
