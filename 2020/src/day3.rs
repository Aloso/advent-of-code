struct Forest {
    width: usize,
    height: usize,
    values: Vec<bool>,
}

impl Forest {
    fn get(&self, x: usize, y: usize) -> bool {
        let x = x % self.width;
        self.values[y * self.width + x]
    }

    fn count_trees(&self, right: usize, down: usize) -> usize {
        (0..self.height / down)
            .filter(|&n| self.get(n * right, n * down))
            .count()
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Forest {
    let width = input.lines().next().unwrap().len();
    let values = input
        .bytes()
        .filter(|&b| b == b'.' || b == b'#')
        .map(|b| b == b'#')
        .collect::<Vec<bool>>();

    Forest {
        width,
        height: values.len() / width,
        values,
    }
}

#[aoc(day3, part1)]
fn part1(forest: &Forest) -> usize {
    forest.count_trees(3, 1)
}

#[aoc(day3, part2)]
fn part2(forest: &Forest) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| forest.count_trees(right, down))
        .product()
}
