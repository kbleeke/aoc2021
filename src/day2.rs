pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut depth = 0;
    let mut position = 0;

    for line in input.lines() {
        let mut components = line.split_ascii_whitespace();
        let direction = components.next().unwrap();
        let amount: i32 = components.next().unwrap().parse()?;

        match direction {
            "forward" => {
                position += amount;
            }
            "up" => {
                depth -= amount;
            }
            "down" => {
                depth += amount;
            }
            _ => {}
        }
    }

    println!("{}", position * depth);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut components = line.split_ascii_whitespace();
        let direction = components.next().unwrap();
        let amount: i32 = components.next().unwrap().parse()?;

        match direction {
            "forward" => {
                position += amount;
                depth += aim * amount;
            }
            "up" => {
                aim -= amount;
            }
            "down" => {
                aim += amount;
            }
            _ => {}
        }
    }

    println!("{}", position * depth);

    Ok(())
}
