use std::{collections::HashSet, io::Read};

use anyhow::{bail, Result};

/// This is an efficient solution with amortized linear runtime,
/// using a `HashSet`.
///
/// ### Usage
/// ```shell
/// cargo run --bin star1 < ./data/star1.txt
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

    for n in input {
        let m = 2020 - n;
        if set.contains(&n) {
            println!("The numbers are {} and {}.", m, n);
            println!("The solution is {}.", m * n);
            return Ok(());
        }
        set.insert(m);
    }

    bail!("No solution found")
}
