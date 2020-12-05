struct BoardingPass {
    row: u8,
    col: u8,
}

impl BoardingPass {
    fn id(&self) -> usize {
        (self.row as usize * 8) + self.col as usize
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(make_boarding_pass).collect()
}

fn make_boarding_pass(input: &str) -> BoardingPass {
    let row = input[0..7]
        .bytes()
        .fold(0, |acc, b| (acc << 1) + if b == b'F' { 0 } else { 1 });
    let col = input[7..10]
        .bytes()
        .fold(0, |acc, b| (acc << 1) + if b == b'L' { 0 } else { 1 });
    BoardingPass { row, col }
}

#[aoc(day5, part1)]
fn part1(values: &[BoardingPass]) -> usize {
    values.iter().map(BoardingPass::id).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(values: &[BoardingPass]) -> usize {
    // enough memory to fit 883 bits
    let mut bitfield = [0u32; 28];

    for value in values {
        let id = value.id();
        bitfield[id / 32] |= 0x8000_0000 >> (id % 32);
    }

    let (i, n) = bitfield
        .iter()
        .map(|&n| n.overflowing_add(1).0)
        .enumerate()
        .find(|&(_, n)| n != 0 && !n.is_power_of_two())
        .unwrap();

    i as usize * 32 + (n - 1).leading_ones() as usize
}
