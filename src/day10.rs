pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut output = 0;

    for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                closing => {
                    let expected = stack.pop();
                    if Some(closing) != expected {
                        output += match closing {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("illegal closing")
                        };
                        break
                    }
                }
            }
        }
    }

    println!("{}", output);

    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut outputs = vec![];

    'lines: for line in input.lines() {
        let mut score = 0u64;
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                closing => {
                    let expected = stack.pop();
                    if Some(closing) != expected {
                        continue 'lines;
                    }
                }
            }
        }
        for c in stack.iter().rev() {
            score *= 5;
            score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }
        outputs.push(score);
    }

    outputs.sort_unstable();
    let len = outputs.len();
    let output = outputs[len / 2];

    println!("{}", output);

    Ok(())
}