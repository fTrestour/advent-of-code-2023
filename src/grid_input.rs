use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GridInput<T> {
    pub max_position: Position,
    pub data: Vec<T>,
}

impl<T> GridInput<T> {
    pub fn find_position<P>(&self, predicate: P) -> Option<(Position, &T)>
    where
        P: FnMut(&&T) -> bool,
    {
        self.data
            .iter()
            .find_position(predicate)
            .map(|(index, el)| (self.position_from_index(index), el))
    }

    pub fn get(&self, position: &Position) -> Result<Option<&T>, ()> {
        Ok(self.data.get(self.position_to_index(position)?))
    }

    pub fn get_mut(&mut self, position: &Position) -> Result<Option<&mut T>, ()> {
        let index = self.position_to_index(position)?;
        Ok(self.data.get_mut(index))
    }

    fn position_from_index(&self, index: usize) -> Position {
        Position::from_index(index, self.max_position.x + 1)
    }

    fn position_to_index(&self, position: &Position) -> Result<usize, ()> {
        if position.x > self.max_position.y || position.y > self.max_position.y {
            return Err(());
        }
        Ok(position.to_index(self.max_position.x + 1))
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Copy, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn from_coordinates(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn from_index(index: usize, line_len: usize) -> Position {
        Position {
            x: index % line_len,
            y: index / line_len,
        }
    }

    pub fn to_index(&self, line_len: usize) -> usize {
        self.y * line_len + self.x
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy)]
pub enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    pub fn to_position(&self, p: &Position) -> Option<Position> {
        match (p.x, p.y, self) {
            (_, 0, Direction::Top) => None,
            (_, _, Direction::Top) => Some(Position::from_coordinates(p.x, p.y - 1)),
            (0, _, Direction::Left) => None,
            (_, _, Direction::Left) => Some(Position::from_coordinates(p.x - 1, p.y)),
            (_, _, Direction::Right) => Some(Position::from_coordinates(p.x + 1, p.y)),
            (_, _, Direction::Bottom) => Some(Position::from_coordinates(p.x, p.y + 1)),
        }
    }

    pub fn list_all() -> Vec<Direction> {
        vec![
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Top => "⬆️",
                Direction::Left => "⬅️",
                Direction::Right => "➡️",
                Direction::Bottom => "⬇️",
            }
        )
    }
}

impl<T> Display for GridInput<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks(self.max_position.x + 1) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
