type Pass = u16;

#[aoc_generator(day5)]
fn input_generator(input: &[u8]) -> Vec<Pass> {
    input.chunks(11).map(make_boarding_pass).collect()
}

fn make_boarding_pass(input: &[u8]) -> Pass {
    let row = input[0..7]
        .iter()
        .copied()
        .fold(0, |acc, b| (acc << 1) + (b == b'B') as Pass);
    let col = input[7..10]
        .iter()
        .copied()
        .fold(0, |acc, b| (acc << 1) + (b == b'R') as Pass);
    (row * 8) + col
}

#[aoc(day5, part1)]
fn part1(passes: &[Pass]) -> Pass {
    passes.iter().copied().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(passes: &[Pass]) -> Pass {
    let iter = passes.iter().copied();
    let (min, max, sum) = iter.fold((Pass::MAX, Pass::MIN, 0), |(min, max, sum), v| {
        (min.min(v), max.max(v), sum + v)
    });

    (min..=max).sum::<Pass>() - sum
}
