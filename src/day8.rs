use std::{
    collections::{HashMap, HashSet},
};

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split('|');
        let _left = parts.next();
        let right = parts.next().unwrap();

        for segment in right.split_ascii_whitespace() {
            if [2, 4, 3, 7].contains(&segment.len()) {
                total += 1;
            }
        }
    }

    println!("{}", total);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split('|');
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let l: HashMap<usize, HashSet<_>> = left
            .split_ascii_whitespace()
            .map(|t| (t.len(), t.chars().collect()))
            .collect();

        let mut n = String::with_capacity(4);
        for o in right
            .split_ascii_whitespace()
            .map(|s| s.chars().collect::<HashSet<_>>())
        {
            let four = l.get(&4).unwrap().intersection(&o).count();
            let two = l.get(&2).unwrap().intersection(&o).count();
            match (o.len(),four, two) {
                (2, _, _) => n += "1",
                (3, _, _) => n += "7",
                (4, _, _) => n += "4",
                (7, _, _) => n += "8",
                (5, 2, _) => n += "2",
                (5, 3, 1) => n += "5",
                (5, 3, 2) => n += "3",
                (6, 4, _) => n += "9",
                (6, 3, 1) => n += "6",
                (6, 3, 2) => n += "0",
                _ => unreachable!("wa")
            }
        }
        total += n.parse::<i32>().unwrap();
    }

    println!("{}", total);

    Ok(())
}
