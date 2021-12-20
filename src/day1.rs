pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut previous = 0;
    let mut num_increases = 0;
    for line in input.split_ascii_whitespace() {
        let depth: u32 = line.parse().expect("parse");

        if depth > previous {
            num_increases += 1;
        }
        previous = depth;
    }

    println!("{}", num_increases - 1);
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let numbers: Vec<u32> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("number");

    let mut previous = 0;
    let mut num_increases = 0;
    for window in numbers.windows(3) {
        let sum: u32 = window.iter().sum();

        if sum > previous {
            num_increases += 1;
        }
        previous = sum;
    }

    println!("{}", num_increases - 1);
    Ok(())
}
