use std::str::FromStr;

mod day_1;
mod day_2;
mod day_5;
mod day_5_part2;

#[derive(Clone)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err("no match"),
        }
    }
}

pub fn solve(day: u8, part: Part, input: &str) -> u32 {
    match (day, part) {
        (1, Part::One) => day_1::solve(input),
        (2, Part::One) => day_2::solve_part1(input),
        (2, Part::Two) => day_2::solve_part2(input),
        (5, Part::One) => day_5::solve_part1(input) as u32,
        (5, Part::Two) => day_5_part2::solve_part2(input) as u32,
        _ => todo!(),
    }
}
