use itertools::Itertools;

#[derive(Debug)]
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

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.data.get(self.position_to_index(position))
    }

    pub fn get_mut(&mut self, position: &Position) -> Option<&mut T> {
        let index = self.position_to_index(position);
        self.data.get_mut(index)
    }

    fn position_from_index(&self, index: usize) -> Position {
        Position::from_index(index, self.max_position.x + 1)
    }

    fn position_to_index(&self, position: &Position) -> usize {
        position.to_index(self.max_position.x + 1)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Copy)]
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
}
