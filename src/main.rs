use std::{fs::File, io::Read};

use advent_of_code_2023::{solve, Part};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: u8,
    part: Part,
}

fn main() {
    let cli = Cli::parse();
    let input = format!("input/day_{}.txt", cli.day);

    let mut file = File::open(input).expect("Could not open input file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read input file");

    let solution = solve(cli.day, cli.part, &input);
    println!("{}", solution);
}
