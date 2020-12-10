use std::collections::HashMap;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|s| {
            let mut s = s.split(" bags contain ");
            let k = s.next().unwrap();
            let v = s
                .next()
                .unwrap()
                .split(", ")
                .filter(|&s| s != "no other bags.")
                .map(|s| {
                    s.trim_end_matches('.')
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag")
                        .trim_start_matches(|c: char| c.is_ascii_digit())
                        .trim()
                })
                .collect::<Vec<_>>();

            (k, (v, false))
        })
        .collect::<HashMap<_, _>>();

    loop {
        let mut altered = vec![];
        for (&k, &(ref v, b)) in &map {
            if b {
                continue;
            }
            if v.iter()
                .any(|&s| s == "shiny gold" || map.get(s).filter(|&(_, b)| *b).is_some())
            {
                altered.push(k);
            }
        }
        if altered.is_empty() {
            break;
        }
        for key in altered {
            map.entry(key).and_modify(|(_, b)| {
                *b = true;
            });
        }
    }

    map.iter().filter(|(_, &(_, b))| b).count()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|s| {
            let mut s = s.split(" bags contain ");
            let k = s.next().unwrap();
            let v = s
                .next()
                .unwrap()
                .split(", ")
                .filter(|&s| s != "no other bags.")
                .map(|s| {
                    let s = s
                        .trim_end_matches('.')
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag")
                        .trim();
                    let i = s.find(|c: char| !c.is_ascii_digit()).unwrap();
                    let n = s[0..i].parse::<i32>().unwrap();
                    let s = s.trim_start_matches(|c: char| {
                        c.is_ascii_digit() || c.is_ascii_whitespace()
                    });

                    (s, n)
                })
                .collect::<Vec<_>>();

            (k, v)
        })
        .collect::<HashMap<_, _>>();

    let mut bags = map["shiny gold"].clone();
    let mut count = 0;

    loop {
        let mut new_bags = vec![];
        for (bag, n) in bags {
            count += n;
            for &(inner, ni) in &map[bag] {
                new_bags.push((inner, ni * n));
            }
        }
        bags = new_bags;
        if bags.is_empty() {
            break;
        }
    }

    count
}
