#[derive(Debug, Copy, Clone)]
struct Answers {
    bits: u32,
}

impl Answers {
    fn set(&mut self, b: u8) {
        self.bits |= 1 << b;
    }
    fn and(self, other: Self) -> Self {
        let bits = self.bits & other.bits;
        Answers { bits }
    }

    fn count_yes(&self) -> u32 {
        self.bits.count_ones()
    }
}

impl From<&str> for Answers {
    fn from(s: &str) -> Self {
        let mut answers = Answers { bits: 0 };
        for b in s.bytes() {
            if b.is_ascii_alphabetic() {
                answers.set(b - b'a');
            }
        }
        answers
    }
}

#[aoc(day6, part1)]
fn part1(data: &str) -> u32 {
    data.split("\n\n")
        .map(|a| Answers::from(a).count_yes())
        .sum()
}

#[aoc(day6, part2)]
fn part2(data: &str) -> u32 {
    data.split("\n\n")
        .map(|s| {
            s.lines()
                .map(Answers::from)
                .fold(Answers { bits: u32::MAX }, Answers::and)
                .count_yes()
        })
        .sum()
}
