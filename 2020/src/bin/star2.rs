use std::{collections::HashSet, io::Read};

use anyhow::{bail, Result};

/// This is an efficient solution with amortized quadratic runtime,
/// similar to the first solution.
///
/// ### Usage
/// ```shell
/// cargo run --bin star2 < ./data/day1.txt
/// ```
fn main() -> Result<()> {
    if atty::is(atty::Stream::Stdin) {
        bail!("Missing input");
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let input: Vec<i64> = input
        .lines()
        .map(str::trim)
        .filter(|&s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()?;

    let mut set = HashSet::new();

    for (i, &n) in input.iter().enumerate() {
        set.insert(n);

        for &m in &input[i + 1..] {
            let l = 2020 - m - n;
            if set.contains(&l) {
                println!("The numbers are {}, {} and {}.", l, m, n);
                println!("The solution is {}.", l * m * n);
                return Ok(());
            }
        }
    }

    bail!("No solution found")
}
