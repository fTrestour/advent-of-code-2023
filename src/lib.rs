mod day_1;
mod day_2;

pub fn solve(day: u8, input: &str) -> u32 {
    match day {
        1 => day_1::solve(input),
        2 => day_2::solve(input),
        _ => todo!(),
    }
}
