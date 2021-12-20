use std::collections::HashMap;

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut parts = input.split("\n\n");
    let start = parts.next().unwrap().trim().as_bytes();
    let mut rules = HashMap::new();
    for rule in  parts.next().unwrap().lines() {
        let rule = rule.as_bytes();
        rules.insert((rule[0], rule[1]), rule[6]);
    }

    let mut current = start.to_owned();
    for _i in 0..10 {
        let mut new = vec![];
        for i in 1..current.len() {
            new.push(current[i-1]);
            if let Some(ins) = rules.get(&(current[i-1], current[i])) {
                new.push(*ins);
            }
        }
        new.push(current[current.len() - 1]);
        current = new;
    }

    let mut counts: HashMap<u8, i32> = HashMap::new();
    for b in current {
        *counts.entry(b).or_default() += 1i32;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut parts = input.split("\n\n");
    let start = parts.next().unwrap().trim().as_bytes();
    let mut rules = HashMap::new();
    for rule in  parts.next().unwrap().lines() {
        let rule = rule.as_bytes();
        rules.insert((rule[0], rule[1]), rule[6]);
    }

    let mut pairs = HashMap::<(u8, u8), u64>::new();
    let mut counts = HashMap::<u8, u64>::new();
    for b in start.windows(2) {
        *pairs.entry((b[0], b[1])).or_default() += 1;
    }
    for b in start {
        *counts.entry(*b).or_default() += 1;
    }

    for _i in 0..40 {
        for (pair, count) in pairs.clone() {
            if let Some(&repl) = rules.get(&pair) {
                *pairs.entry(pair).or_default() -= count;
                *pairs.entry((pair.0, repl)).or_default() += count;
                *pairs.entry((repl, pair.1)).or_default() += count;
                *counts.entry(repl).or_default() += count;
            }
        }
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);

    Ok(())
}