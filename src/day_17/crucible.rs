use std::fmt::Display;

use itertools::Itertools;

use crate::grid_input::{Direction, GridInput, Position};

use super::block::Block;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Crucible {
    pub heat_loss: usize,
    pub position: Position,
    pub grid: GridInput<Block>,
    pub previous_moves: Vec<Direction>,
}

impl Crucible {
    pub fn available_directions(&self) -> Vec<Direction> {
        let forbidden_directions = &self.forbidden_directions();

        let mut result = vec![];
        for direction in Direction::list_all() {
            if let Some(position) = direction.to_position(&self.position) {
                if let Ok(Some(block)) = self.grid.get(&position) {
                    if !forbidden_directions.contains(&direction) && block.direction.is_none() {
                        result.push(direction);
                    }
                }
            }
        }
        result
    }

    fn forbidden_directions(&self) -> Vec<Direction> {
        let mut result = vec![];

        let last_move = self.previous_moves.last();
        if let Some(direction) = last_move {
            result.push(direction.reverse());
        }

        let mut last_moves = self
            .previous_moves
            .iter()
            .rev()
            .take(3)
            .map(|direction| *direction)
            .collect_vec();
        let has_3_moves = last_moves.len() == 3;
        last_moves.dedup();
        if has_3_moves && last_moves.len() == 1 {
            let repeated_move = *last_moves.first().unwrap();
            result.push(repeated_move);
        }

        result
    }

    pub fn r#move(&mut self, direction: &Direction) {
        let new_position = direction.to_position(&self.position).unwrap();
        self.position = new_position;

        let new_block = self.grid.get_mut(&new_position).unwrap().unwrap();
        self.heat_loss = self.heat_loss + new_block.heat_loss;

        new_block.direction = Some(*direction);

        self.previous_moves.push(*direction);
    }

    pub fn distance(&self) -> usize {
        9 * (self.grid.max_position.x - self.position.x + self.grid.max_position.y
            - self.position.y)
    }
}

impl From<GridInput<Block>> for Crucible {
    fn from(value: GridInput<Block>) -> Self {
        Self {
            heat_loss: Default::default(),
            position: Default::default(),
            grid: value.clone(),
            previous_moves: Default::default(),
        }
    }
}

impl Display for Crucible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Heatloss: {}", self.heat_loss)?;
        writeln!(f, "Position: {}", self.position)?;

        writeln!(f, "{}", self.grid)?;
        Ok(())
    }
}
