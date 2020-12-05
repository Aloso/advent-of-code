#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(make_boarding_pass).collect()
}

fn make_boarding_pass(input: &str) -> usize {
    let row = input[0..7]
        .bytes()
        .fold(0, |acc, b| (acc << 1) + (b == b'B') as usize);
    let col = input[7..10]
        .bytes()
        .fold(0, |acc, b| (acc << 1) + (b == b'R') as usize);
    (row * 8) + col
}

#[aoc(day5, part1)]
fn part1(passes: &[usize]) -> usize {
    passes.iter().copied().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(passes: &[usize]) -> usize {
    // enough memory to fit 883 bits
    let mut bitfield = [0u32; 28];

    for &pass in passes {
        bitfield[pass / 32] |= 0x8000_0000 >> (pass % 32);
    }

    let (i, n) = bitfield
        .iter()
        .map(|&n| n.overflowing_add(1).0)
        .enumerate()
        .find(|&(_, n)| n != 0 && !n.is_power_of_two())
        .unwrap();

    i as usize * 32 + (n - 1).leading_ones() as usize
}
