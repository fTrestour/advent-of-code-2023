use itertools::Itertools;

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

    fn position_from_index(&self, index: usize) -> Position {
        Position::from_index(index, self.max_position.x + 1)
    }

    fn position_to_index(&self, position: &Position) -> usize {
        position.to_index(self.max_position.x + 1)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd)]
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
