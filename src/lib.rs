use std::str::FromStr;

mod day_1;
mod day_2;
mod day_4;
mod day_5;
mod day_5_part2;
mod day_6;
mod day_8;
mod day_8_part2;
mod day_9;

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
        (1, Part::One) => day_1::solve_part1(input),
        (2, Part::One) => day_2::solve_part1(input),
        (2, Part::Two) => day_2::solve_part2(input),
        (5, Part::One) => day_5::solve_part1(input) as u32,
        (5, Part::Two) => day_5_part2::solve_part2(input) as u32,
        (6, Part::One) => day_6::solve_part1(input) as u32,
        (6, Part::Two) => day_6::solve_part2(input) as u32,
        (4, Part::One) => day_4::solve_part1(input),
        (4, Part::Two) => day_4::solve_part2(input),
        (1, Part::Two) => day_1::solve_part2(input),
        (8, Part::One) => day_8::solve_part1(input),
        (8, Part::Two) => day_8_part2::solve(input) as u32,
        (9, Part::One) => day_9::solve_part1(input) as u32,
        (9, Part::Two) => day_9::solve_part2(input) as u32,
        _ => todo!(),
    }
}
