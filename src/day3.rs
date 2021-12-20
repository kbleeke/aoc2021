pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut bitcount = vec![0; 0];
    let lines = input.lines().count();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if bitcount.len() <= i {
                bitcount.resize(i + 1, 0);
            }

            if c == '1' {
                bitcount[i] += 1;
            }
        }
    }

    println!("{:?}", bitcount);

    let mut gamma = 0;
    let mut epsilon = 0;
    for (_, count) in bitcount.into_iter().enumerate() {
        let set = count > lines / 2;

        gamma = (gamma << 1) | (set as i32);
        epsilon = (epsilon << 1) | (!set as i32);
    }

    dbg!(gamma, epsilon);
    println!("{}", gamma * epsilon);
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut bit_index = 0;
    let mut oxy_values: Vec<_> = input.lines().map(|s| s).collect();
    loop {
        // println!("{:?}", oxy_values);
        if oxy_values.len() <= 1 {
            break;
        }

        let most_common: usize = oxy_values
            .iter()
            .map(|line| {
                if line.as_bytes()[bit_index] == b'1' {
                    1
                } else {
                    0
                }
            })
            .sum();
        // dbg!(most_common);
        let most_common = if most_common as f32 >= ((oxy_values.len() as f32) / 2.0) {
            b'1'
        } else {
            b'0'
        };

        // dbg!(most_common);
        oxy_values = oxy_values
            .into_iter()
            .filter(|n| n.as_bytes()[bit_index] == most_common)
            .collect();
        bit_index += 1;
    }

    println!("{}", oxy_values[0]);

    let mut bit_index = 0;
    let mut co2_values: Vec<_> = input.lines().map(|s| s).collect();
    loop {
        // println!("{:?}", co2_values);
        if co2_values.len() <= 1 {
            break;
        }

        let most_common: usize = co2_values
            .iter()
            .map(|line| {
                if line.as_bytes()[bit_index] == b'1' {
                    1
                } else {
                    0
                }
            })
            .sum();
        // dbg!(most_common);
        let most_common = if most_common as f32 >= ((co2_values.len() as f32) / 2.0) {
            b'0'
        } else {
            b'1'
        };

        // dbg!(most_common);
        co2_values = co2_values
            .into_iter()
            .filter(|n| n.as_bytes()[bit_index] == most_common)
            .collect();
        bit_index += 1;
    }

    println!("{}", co2_values[0]);


    let oxy: i32 = i32::from_str_radix(oxy_values[0], 2)?;
    let co2: i32 = i32::from_str_radix(co2_values[0], 2)?;

    println!("{}", oxy * co2);

    Ok(())
}
