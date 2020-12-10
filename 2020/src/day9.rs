use std::collections::VecDeque;

#[aoc_generator(day9)]
fn input_generator(data: &str) -> Vec<u64> {
    data.lines().map(|n| n.parse().unwrap()).collect()
}

// 556543474
#[aoc(day9, part1)]
fn part1(numbers: &[u64]) -> u64 {
    let (i, _) = numbers
        .windows(26)
        .enumerate()
        .find(|(_, nums)| {
            let num = nums[25];
            for (n, &i) in nums[0..25].iter().enumerate() {
                for &j in &nums[n..25] {
                    if i + j == num && i != j {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();

    numbers[i + 25]
}

// 76096372
#[aoc(day9, part2)]
fn part2(numbers: &[u64]) -> u64 {
    let required = part1(numbers);
    let mut sums = VecDeque::new();

    for (i, &n) in numbers.iter().enumerate() {
        for (j, s) in sums.iter_mut().enumerate() {
            *s += n;
            if *s == required {
                let j = i + j - sums.len();
                let (min, max) = numbers[j..=i]
                    .iter()
                    .fold((u64::MAX, u64::MIN), |(min, max), &n| {
                        (min.min(n), max.max(n))
                    });
                return min + max;
            }
        }
        if let Some((i, _)) = sums.iter().enumerate().find(|&(_, &x)| x < required) {
            if i > 0 {
                let _ = sums.drain(0..i);
            }
        }
        sums.push_back(n);
    }

    panic!("Not found")
}
