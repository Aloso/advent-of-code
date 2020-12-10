#[derive(Debug, Copy, Clone)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[aoc_generator(day8)]
fn input_generator(data: &str) -> Vec<Op> {
    data.lines()
        .map(|s| {
            let n = s[4..].parse().unwrap();
            match &s[..3] {
                "nop" => Op::Nop(n),
                "acc" => Op::Acc(n),
                "jmp" => Op::Jmp(n),
                _ => panic!("Unexpected instruction"),
            }
        })
        .collect()
}

fn part1_impl(instructions: &[Op]) -> (Vec<bool>, i32, bool) {
    let mut visited = vec![false; instructions.len()];
    let mut acc = 0;
    let mut pc = 0;

    loop {
        let i = pc as usize;
        if i == instructions.len() {
            break (visited, acc, true);
        } else if visited[i] {
            break (visited, acc, false);
        }
        visited[i] = true;

        match instructions[i] {
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Op::Jmp(n) => {
                pc += n;
            }
        }
    }
}

#[aoc(day8, part1)]
fn part1(instructions: &[Op]) -> i32 {
    part1_impl(instructions).1
}

#[derive(Debug, Copy, Clone)]
enum Conn {
    Normal(usize),
    Alt(usize),
}

type Connections = Vec<Conn>;

fn part2_impl(
    instructions: &[Op],
    reachable: &[bool],
    visited: &mut [bool],
    back_links: &[Connections],
    i: usize,
) -> Option<usize> {
    if visited[i] {
        return None;
    }

    // println!(" -> {:>3}  {:>4?}  {:?}", i, instructions[i], back_links[i]);

    for &link in &back_links[i] {
        if let Conn::Alt(n) = link {
            if reachable[n] {
                return Some(n);
            }
        }
    }
    for &link in &back_links[i] {
        if let Conn::Normal(n) = link {
            if let Some(n) = part2_impl(instructions, reachable, visited, back_links, n) {
                return Some(n);
            }
        }
    }

    None
}

#[aoc(day8, part2)]
fn part2(instructions: &[Op]) -> i32 {
    let (reachable, ..) = part1_impl(instructions);
    let mut back_links: Vec<Connections> = vec![Connections::default(); instructions.len()];

    for (i, &op) in instructions.iter().enumerate() {
        match op {
            Op::Nop(n) => {
                if let Some(c) = back_links.get_mut(i + 1) {
                    c.push(Conn::Normal(i));
                }
                if n != 1 {
                    if let Some(c) = back_links.get_mut(i + n as usize) {
                        c.push(Conn::Alt(i));
                    }
                }
            }
            Op::Acc(_) => {
                if let Some(c) = back_links.get_mut(i + 1) {
                    c.push(Conn::Normal(i));
                }
            }
            Op::Jmp(n) => {
                if let Some(c) = back_links.get_mut(i + n as usize) {
                    c.push(Conn::Normal(i));
                }
                if n != 1 {
                    if let Some(c) = back_links.get_mut(i + 1) {
                        c.push(Conn::Alt(i));
                    }
                }
            }
        }
    }

    let mut visited = vec![false; instructions.len()];
    let i = instructions.len() - 1;

    let invalid_op = part2_impl(instructions, &reachable, &mut visited, &back_links, i).unwrap();
    let mut instructions = instructions.to_vec();
    instructions[invalid_op] = match instructions[invalid_op] {
        Op::Nop(n) => Op::Jmp(n),
        Op::Acc(_) => panic!("acc instead of nop or jmp"),
        Op::Jmp(n) => Op::Nop(n),
    };

    let (_, acc, finished) = part1_impl(&instructions);
    assert!(finished);
    acc
}
