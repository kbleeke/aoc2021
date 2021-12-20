pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut fishes = input
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    println!("begin: {} fish", fishes.len());

    for day in 0..80 {
        println!("day {}: {} fish", day, fishes.len());
        let new_fish = fishes.iter_mut().fold(0, |mut acc, fish| {
            if *fish == 0 {
                acc += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
            acc
        });
        for _fish in 0..new_fish {
            fishes.push(8);
        }
    }

    println!("{}", fishes.len());

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut buckets = [0u64; 9];

    for fish in input.split(",").map(str::parse::<usize>) {
        buckets[fish?] += 1;
    }

    let total: u64 = buckets.iter().sum();
    println!("begin: {} fish", total);

    for _day in 0..256 {
        // let new_fish = buckets[0];
        // for i in 0..8 {
        //     buckets[i] = buckets[i + 1];
        // }


        // buckets[6] += new_fish;
        // buckets[8] = new_fish;
        buckets.rotate_left(1);
        buckets[6] += buckets[8];
    }

    let total: u64 = buckets.iter().sum();
    println!("fishes: {}", total);

    Ok(())
}
