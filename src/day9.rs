use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut output = 0;
    let mut grid = vec![];
    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(line);
    }

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if x != 0 && grid[y][x-1] <= *c {
                continue;
            }
            if y != 0 && grid[y-1][x] <= *c {
                continue;
            }
            if let Some(xv) = grid[y].get(x+1) {
                if xv <= c {
                    continue;
                }
            }
            if let Some(yline) = grid.get(y+1){
                if yline[x] <= *c {
                    continue;
                }
            }
            output += 1 + c;
        }
    }

    println!("{}", output);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    // let mut output = 0;
    let mut grid = vec![];
    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(line);
    }

    let mut low_points = vec![];

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if x != 0 && grid[y][x-1] <= *c {
                continue;
            }
            if y != 0 && grid[y-1][x] <= *c {
                continue;
            }
            if let Some(xv) = grid[y].get(x+1) {
                if xv <= c {
                    continue;
                }
            }
            if let Some(yline) = grid.get(y+1){
                if yline[x] <= *c {
                    continue;
                }
            }
            low_points.push((x, y));
        }
    }

    // println!("{:?}", low_points);

    let mut basin_sizes = vec![];
    for (x,y) in low_points {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<_> = std::iter::once((x, y)).collect();
        while let Some((x, y)) = queue.pop_front() {
            let v = grid[y][x];
            if v == 9 {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            if x != 0 && grid[y][x-1] >= v {
                queue.push_back((x-1,y));
            }
            if y != 0 && grid[y-1][x] >= v {
                queue.push_back((x, y-1));
            }
            if let Some(c) = grid[y].get(x+1) {
                if *c >= v {
                    queue.push_back((x+1, y));
                }
            }
            if let Some(c) = grid.get(y+1).map(|l| l[x]) {
                if c >= v {
                    queue.push_back((x, y+1));
                }
            }
        }
        basin_sizes.push(visited.len());
    }
    basin_sizes.sort_unstable();
    println!("{:?}", basin_sizes);

    let prod: usize = basin_sizes.iter().rev().take(3).product();

    println!("{}", prod);

    Ok(())
}