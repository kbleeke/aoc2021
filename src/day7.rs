pub fn part1(input: &str) -> anyhow::Result<()> {
    let positions = input
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let min = positions.iter().min().copied().unwrap();
    let max = positions.iter().max().copied().unwrap();


    let mut min_fuel = i32::MAX;
    for i in min..max {
        let fuel = positions.iter().map(|p| (i - p).abs()).sum();
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    println!("{}", min_fuel);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let positions = input
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let min = positions.iter().min().copied().unwrap();
    let max = positions.iter().max().copied().unwrap();

    dbg!(min, max);
    let mut min_fuel = i32::MAX;
    for i in min..max {
        // println!("{}", i);
        let fuel = positions.iter().map(|p| {
            let d = (i - p).abs();
            d * (d+1) / 2
        }).sum();
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    println!("{}", min_fuel);

    Ok(())
}