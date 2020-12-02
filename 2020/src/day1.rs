use std::collections::HashSet;

type Num = i64;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Num> {
    input
        .lines()
        .map(str::trim)
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Num]) -> Num {
    let mut set = HashSet::new();

    for &n in input {
        let m = 2020 - n;
        set.insert(m);

        if set.contains(&n) {
            return m * n;
        }
    }
    panic!("No solution found");
}

#[aoc(day1, part2)]
fn part2(input: &[Num]) -> Num {
    let mut set = HashSet::new();

    for (i, &n) in input.iter().enumerate() {
        set.insert(n);

        for &m in &input[i + 1..] {
            let l = 2020 - m - n;
            if set.contains(&l) {
                return l * m * n;
            }
        }
    }
    panic!("No solution found")
}
