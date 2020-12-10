#[aoc_generator(day10)]
fn input_generator(data: &str) -> Vec<u16> {
    let iter = data.lines().map(|s| s.parse().unwrap());
    let mut values: Vec<u16> = std::iter::once(0u16).chain(iter).collect();
    values.sort_unstable();
    values
}

#[aoc(day10, part1)]
fn part1(adapters: &[u16]) -> u16 {
    let (one, three) = adapters
        .windows(2)
        .map(|a| a[1] - a[0])
        .fold((0, 0), |(one, three), n| match n {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => (one, three),
        });

    one * (three + 1)
}

#[aoc(day10, part2)]
fn part2(adapters: &[u16]) -> u64 {
    let mut possibilities = vec![0; adapters.len()];
    possibilities[adapters.len() - 1] = 1;

    for i in (0..possibilities.len() - 1).rev() {
        let head = adapters[i];
        let tail = &adapters[i + 1..];

        for j in 0..3 {
            if tail.get(j).filter(|&&n| n - head <= 3).is_some() {
                possibilities[i] += possibilities[i + j + 1];
            }
        }
    }

    possibilities[0]
}
