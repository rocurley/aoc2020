use aoc2020::*;
use argh::FromArgs;
use cpuprofiler::PROFILER;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// which problem to solve
    #[argh(option, short = 'p')]
    problem: String,

    /// path for the input
    #[argh(positional)]
    input_path: String,

    /// enable profiling
    #[argh(switch)]
    profile: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let file = fs::File::open(args.input_path).expect("couldn't read input");
    let profiler = if args.profile {
        let mut profiler = PROFILER.lock().unwrap();
        profiler
            .start(format!("profiling/{}", args.problem.as_str()))
            .unwrap();
        let now = Instant::now();
        Some((profiler, now))
    } else {
        None
    };
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
        "day8" => {
            day8::solve1(&input);
            day8::solve2(&input);
        }
        "day9" => {
            day9::solve2(&input);
        }
        "day10" => {
            day10::solve1(&input);
            day10::solve2(&input);
        }
        "day11" => {
            day11::solve1(&input);
            day11::solve2(&input);
        }
        "day12" => {
            day12::solve1(&input);
            day12::solve2(&input);
        }
        "day13" => {
            day13::solve1(&input);
            day13::solve2(&input);
        }
        "day14" => {
            day14::solve1(&input);
            day14::solve2(&input);
        }
        "day15" => {
            day15::solve1(&input);
        }
        "day16" => {
            day16::solve1(&input);
            day16::solve2(&input);
        }
        "day17" => {
            day17::solve1(&input);
        }
        _ => panic!("Unexpected problem name"),
    }
    if let Some((mut p, start)) = profiler {
        let t = start.elapsed();
        p.stop().unwrap();
        println!("{:?}", t);
    }
}
