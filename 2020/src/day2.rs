use regex::Regex;

struct Data {
    c: char,
    min: usize,
    max: usize,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Data> {
    let regex = Regex::new("^(.*?)-(.*?) (.*?): (.*?)$").unwrap();

    input
        .lines()
        .filter(|&s| !s.is_empty())
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            assert_eq!(captures[3].chars().nth(1), None);

            Data {
                c: captures[3].chars().next().unwrap(),
                min: captures[1].parse().unwrap(),
                max: captures[2].parse().unwrap(),
                password: captures[4].into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(data: &[Data]) -> usize {
    data.iter()
        .filter(|&data| {
            let count = data.password.chars().filter(|&c| c == data.c).count();
            count >= data.min && count <= data.max
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(data: &[Data]) -> usize {
    data.iter()
        .filter(|&data| {
            let mut chars = data.password.chars();
            let char1 = chars.nth(data.min - 1).unwrap();
            let char2 = chars.nth(data.max - data.min - 1).unwrap();

            (char1 == data.c) != (char2 == data.c)
        })
        .count()
}
