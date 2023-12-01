use std::{fs::File, io::Read, path::PathBuf};

use advent_of_code_2023::solve;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: u8,
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut file = File::open(cli.input).expect("Could not open input file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read input file");

    let solution = solve(cli.day, &input);
    println!("{}", solution);
}
