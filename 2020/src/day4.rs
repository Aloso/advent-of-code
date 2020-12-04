use std::{convert::TryFrom, fmt::Debug, ops::RangeInclusive};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Value1 {
    CountryId,
    Other,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Value2 {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl TryFrom<&str> for Value1 {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(':');
        match parts.next().unwrap() {
            "cid" => Ok(Self::CountryId),
            "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Value2 {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        let mut iter = s.split(':');
        match iter.next().unwrap() {
            "byr" => then(number_in(iter, 1920..=2002), Self::BirthYear),
            "iyr" => then(number_in(iter, 2010..=2020), Self::IssueYear),
            "eyr" => then(number_in(iter, 2020..=2030), Self::ExpirationYear),
            "hgt" => {
                let s = iter.next().unwrap();
                let valid = if s.ends_with("cm") {
                    number_in(s.split("cm").next().into_iter(), 150..=193)
                } else if s.ends_with("in") {
                    number_in(s.split("in").next().into_iter(), 59..=76)
                } else {
                    false
                };
                then(valid, Self::Height)
            }
            "hcl" => {
                let s = iter.next().unwrap().strip_prefix('#').ok_or(())?;
                let n = u32::from_str_radix(s, 16).ok().ok_or(())?;
                if (0x100..=0xFFF).contains(&n) || (0x100000..=0xFFFFFF).contains(&n) {
                    Ok(Self::HairColor)
                } else {
                    Err(())
                }
            }
            "ecl" => then(
                matches!(
                    iter.next().unwrap(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                ),
                Self::EyeColor,
            ),
            "pid" => {
                let s = iter.next().unwrap();
                let valid = s.len() == 9 && s.bytes().all(|b| b.is_ascii_digit());
                then(valid, Self::PassportId)
            }
            "cid" => Ok(Self::CountryId),
            _ => Err(()),
        }
    }
}

fn number<'a>(mut iter: impl Iterator<Item = &'a str>) -> Option<i32> {
    iter.next().unwrap().parse::<i32>().ok()
}

fn number_in<'a>(iter: impl Iterator<Item = &'a str>, range: RangeInclusive<i32>) -> bool {
    number(iter).filter(|n| range.contains(n)).is_some()
}

fn then<T: Debug>(b: bool, value: T) -> Result<T, ()> {
    if b {
        Ok(value)
    } else {
        Err(())
    }
}

#[aoc_generator(day4, part1)]
fn input_generator_part1(input: &str) -> Vec<Vec<Value1>> {
    input_generator(input)
}

#[aoc_generator(day4, part2)]
fn input_generator_part2(input: &str) -> Vec<Vec<Value2>> {
    input_generator(input)
}

fn input_generator<'a, T: TryFrom<&'a str, Error = ()>>(s: &'a str) -> Vec<Vec<T>> {
    s.split("\n\n")
        .filter_map(|vals| {
            vals.split_whitespace()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<T>, ()>>()
                .ok()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(values: &[Vec<Value1>]) -> usize {
    values
        .iter()
        .filter(|&p| p.len() >= 8 || (p.len() == 7 && !p.contains(&Value1::CountryId)))
        .count()
}

#[aoc(day4, part2)]
fn part2(values: &[Vec<Value2>]) -> usize {
    values
        .iter()
        .filter(|&p| p.len() >= 8 || (p.len() == 7 && !p.contains(&Value2::CountryId)))
        .count()
}
