use aoc2020::*;
use argh::FromArgs;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// which problem to solve
    #[argh(option, short = 'p')]
    problem: String,

    /// path for the input
    #[argh(positional)]
    input_path: String,
}

fn main() {
    let args: Args = argh::from_env();
    let file = fs::File::open(args.input_path).expect("couldn't read input");
    let input = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("coudln't read line");
    match args.problem.as_str() {
        "day1" => {
            day1::solve1(&input);
            day1::solve2(&input);
        }
        "day2" => {
            day2::solve1(&input);
            day2::solve2(&input);
        }
        "day3" => {
            day3::solve1(&input);
        }
        "day4" => {
            day4::solve1(&input);
        }
        "day5" => {
            day5::solve1(&input);
            day5::solve2(&input);
        }
        "day6" => {
            day6::solve1(&input);
        }
        "day7" => {
            day7::solve1(&input);
        }
        _ => panic!("Unexpected problem name"),
    }
}
