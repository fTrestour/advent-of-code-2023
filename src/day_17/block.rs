use std::fmt::Display;

use crate::grid_input::Direction;

#[derive(Clone)]
pub struct Block {
    pub heat_loss: usize,
    pub direction: Option<Direction>,
}

impl From<usize> for Block {
    fn from(value: usize) -> Self {
        Block {
            heat_loss: value,
            direction: None,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.direction {
                None => self.heat_loss.to_string(),
                Some(direction) => direction.to_string(),
            }
        )
    }
}
