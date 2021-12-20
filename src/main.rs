use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day14;

fn time(input: &str, f: fn(&str) -> anyhow::Result<()>) {
    let start = Instant::now();

    f(input).unwrap();

    let elapsed = start.elapsed();
    let time = elapsed.as_secs_f64() * 1000.0;
    println!("Elapsed: {:.2}ms", time);
}

fn main() {
    let mut args = std::env::args();

    let day: u32 = args.nth(1).expect("day").parse().expect("number");
    let part: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let input = args.next().expect("input");

    let input = std::fs::read_to_string(&input).expect("input file");

    let f = match (day, part) {
        (1, 1) => day1::part1,
        (1, 2) => day1::part2,
        (2, 1) => day2::part1,
        (2, 2) => day2::part2,
        (3, 1) => day3::part1,
        (3, 2) => day3::part2,
        (6, 1) => day6::part1,
        (6, 2) => day6::part2,
        (7, 1) => day7::part1,
        (7, 2) => day7::part2,
        (8, 1) => day8::part1,
        (8, 2) => day8::part2,
        (9, 1) => day9::part1,
        (9, 2) => day9::part2,
        (10, 1) => day10::part1,
        (10, 2) => day10::part2,
        (14, 1) => day14::part1,
        (14, 2) => day14::part2,
        _ => {
            println!("day {} part {} not available", day, part);
            return;
        }
    };

    time(&input, f);
}
