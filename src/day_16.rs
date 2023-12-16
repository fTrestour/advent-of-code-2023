use crate::grid_input::{Direction, GridInput, Position};
use core::fmt::Debug;
use itertools::Itertools;
use std::fmt::Display;

pub fn solve_part1(input: &str) -> usize {
    let max_position = Position::from_coordinates(
        input.lines().next().unwrap().len() - 1,
        input.lines().count() - 1,
    );
    let mut grid = GridInput {
        max_position,
        data: input
            .lines()
            .flat_map(|line| line.chars())
            .map(Cell::from)
            .collect_vec(),
    };

    let mut beams_queue = vec![Beam {
        direction: Direction::Right,
        position: Position::from_coordinates(0, 0),
    }];
    while let Some(mut beam) = beams_queue.pop() {
        if let Ok(new_beam) = beam.progress(&mut grid) {
            beams_queue.push(beam);

            if let Some(new_beam) = new_beam {
                beams_queue.push(new_beam)
            };
        };

        println!("{}", grid);
    }

    grid.count_energized_cells()
}

struct Cell {
    content: CellType,
    is_energized: bool,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Cell {
            content: CellType::from(value),
            is_energized: false,
        }
    }
}

impl GridInput<Cell> {
    pub fn count_energized_cells(&self) -> usize {
        self.data.iter().filter(|cell| cell.is_energized).count()
    }
}

impl Display for GridInput<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks(self.max_position.x + 1) {
            for cell in row {
                let symbol = if cell.is_energized {
                    "#".to_owned()
                } else {
                    cell.content.to_string()
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum CellType {
    EmptySpace,
    SlashReflector,
    AntiSlashReflector,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellType::EmptySpace => '.',
                CellType::SlashReflector => '/',
                CellType::AntiSlashReflector => '\\',
                CellType::VerticalSplitter => '|',
                CellType::HorizontalSplitter => '-',
            }
        )
    }
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => CellType::EmptySpace,
            '/' => CellType::SlashReflector,
            '\\' => CellType::AntiSlashReflector,
            '|' => CellType::VerticalSplitter,
            '-' => CellType::HorizontalSplitter,
            _ => panic!("Invalid cell value"),
        }
    }
}

#[derive(Debug)]
struct Beam {
    direction: Direction,
    position: Position,
}

impl Beam {
    pub fn progress(&mut self, grid: &mut GridInput<Cell>) -> Result<Option<Self>, ()> {
        if let Some(new_position) = self.direction.to_position(&self.position) {
            self.position = new_position;
            let cell = grid.get_mut(&self.position);
            if let Some(cell) = cell {
                if cell.is_energized {
                    Err(())
                } else {
                    cell.is_energized = true;
                    println!("Cell {:?} energized!", new_position);
                    Ok(self.encounters(&cell.content))
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn encounters(&mut self, cell_type: &CellType) -> Option<Self> {
        match (cell_type, self.direction) {
            (CellType::SlashReflector, Direction::Top)
            | (CellType::AntiSlashReflector, Direction::Bottom) => {
                self.direction = Direction::Right;
                None
            }
            (CellType::SlashReflector, Direction::Right)
            | (CellType::AntiSlashReflector, Direction::Left) => {
                self.direction = Direction::Top;
                None
            }
            (CellType::SlashReflector, Direction::Bottom)
            | (CellType::AntiSlashReflector, Direction::Top) => {
                self.direction = Direction::Left;
                None
            }
            (CellType::SlashReflector, Direction::Left)
            | (CellType::AntiSlashReflector, Direction::Right) => {
                self.direction = Direction::Bottom;
                None
            }
            (CellType::VerticalSplitter, Direction::Left)
            | (CellType::VerticalSplitter, Direction::Right) => {
                self.direction = Direction::Top;
                Some(Beam {
                    direction: Direction::Bottom,
                    position: self.position,
                })
            }
            (CellType::HorizontalSplitter, Direction::Top)
            | (CellType::HorizontalSplitter, Direction::Bottom) => {
                self.direction = Direction::Left;
                Some(Beam {
                    direction: Direction::Right,
                    position: self.position,
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(
            solve_part1(
                ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|...."
            ),
            46
        )
    }
}
