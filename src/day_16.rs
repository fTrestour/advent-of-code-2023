use crate::grid_input::{Direction, GridInput, Position};
use core::fmt::Debug;
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

pub fn solve_part1(input: &str) -> usize {
    let max_position = Position::from_coordinates(
        input.lines().next().unwrap().len() - 1,
        input.lines().count() - 1,
    );
    let grid = GridInput {
        max_position,
        data: input
            .lines()
            .flat_map(|line| line.chars())
            .map(Cell::from)
            .collect_vec(),
    };

    let starting_point = Position::from_coordinates(0, 0);
    let starting_direction = Direction::Right;

    solve(&mut grid.clone(), starting_point, starting_direction)
}

pub fn solve_part2(input: &str) -> usize {
    let max_position = Position::from_coordinates(
        input.lines().next().unwrap().len() - 1,
        input.lines().count() - 1,
    );
    let grid = GridInput {
        max_position,
        data: input
            .lines()
            .flat_map(|line| line.chars())
            .map(Cell::from)
            .collect_vec(),
    };

    let x_range = 0..max_position.x;
    let y_range = 0..max_position.y;

    let top_row = x_range
        .clone()
        .map(|x| (Position::from_coordinates(x, 0), Direction::Bottom));
    let bottom_row = x_range.map(|x| {
        (
            Position::from_coordinates(x, max_position.y),
            Direction::Top,
        )
    });
    let left_column = y_range
        .clone()
        .map(|y| (Position::from_coordinates(0, y), Direction::Right));
    let right_column = y_range.map(|y| {
        (
            Position::from_coordinates(max_position.x, y),
            Direction::Left,
        )
    });

    let total_iterations = 2 * max_position.x + 2 * max_position.y;
    let mut current_iteration = 0;

    let inputs = top_row
        .chain(bottom_row)
        .chain(left_column)
        .chain(right_column);

    let mut max = 0;
    for (starting_point, direction) in inputs {
        let result = solve(&mut grid.clone(), starting_point, direction);
        current_iteration = current_iteration + 1;
        max = max.max(result);
        println!(
            "Iteration {}/{} : {} energized cells, {} current max",
            current_iteration, total_iterations, result, max
        );
    }
    max
}

fn solve(
    mut grid: &mut GridInput<Cell>,
    starting_point: Position,
    starting_direction: Direction,
) -> usize {
    grid.get_mut(&starting_point).unwrap().unwrap().is_energized = true;
    let mut beams_queue = BeamQueue(vec![Beam::new(starting_direction, starting_point)].into());
    let mut loops_count = 0;
    let mut energized_cells_count = grid.count_energized_cells();
    while let Some(mut beam) = beams_queue.0.pop_front() {
        // println!("Queue: {}", beams_queue);
        // println!("{}", beam);

        if let Ok(new_beam) = beam.progress(&mut grid) {
            beams_queue.0.push_back(beam);

            if let Some(new_beam) = new_beam {
                beams_queue.0.push_back(new_beam)
            };
            // println!("{}", grid);

            loops_count = loops_count + 1;
            if loops_count % 200000 == 0 {
                // println!("Loop number {}", loops_count);
                if grid.count_energized_cells() - energized_cells_count == 0 {
                    break;
                }
                energized_cells_count = grid.count_energized_cells();
            }
        };
    }

    energized_cells_count
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Debug, Clone)]
struct Beam {
    direction: Direction,
    position: Position,
    already_hit: Vec<Position>,
}

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.position)
    }
}

impl Beam {
    pub fn new(direction: Direction, position: Position) -> Self {
        Beam {
            direction,
            position,
            already_hit: vec![],
        }
    }

    pub fn progress(&mut self, grid: &mut GridInput<Cell>) -> Result<Option<Self>, ()> {
        self.position = self.direction.to_position(&self.position).ok_or(())?;

        let cell = grid.get_mut(&self.position)?.ok_or(())?;

        cell.is_energized = true;
        self.already_hit.push(self.position);

        Ok(self.encounters(&cell.content))
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

                let mut new_beam = self.clone();
                new_beam.direction = Direction::Bottom;
                Some(new_beam)
            }
            (CellType::HorizontalSplitter, Direction::Top)
            | (CellType::HorizontalSplitter, Direction::Bottom) => {
                self.direction = Direction::Left;

                let mut new_beam = self.clone();
                new_beam.direction = Direction::Right;
                Some(new_beam)
            }
            _ => None,
        }
    }
}

struct BeamQueue(VecDeque<Beam>);
impl Display for BeamQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for beam in self.0.iter() {
            write!(f, "{} ", beam)?;
        }
        Ok(())
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

    #[test]
    fn test_solve_part2() {
        assert_eq!(
            solve_part2(
                ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|...."
            ),
            51
        )
    }
}
