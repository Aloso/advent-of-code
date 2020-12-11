type Num = isize;
type PlaceIndex = (Num, Num);

#[derive(Debug, Clone)]
struct Seats {
    places: Vec<Place>,
    width: Num,
    height: Num,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Place {
    Empty,
    Occupied,
    Floor,
    Outside,
}

impl Seats {
    fn new(places: Vec<Place>, width: Num, height: Num) -> Seats {
        Self {
            places,
            width,
            height,
        }
    }

    #[inline]
    fn get(&self, x: Num, y: Num) -> Place {
        self.places[(x + y * self.width) as usize]
    }

    #[inline]
    fn get_mut(&mut self, x: Num, y: Num) -> &mut Place {
        &mut self.places[(x + y * self.width) as usize]
    }

    const DIRECTIONS: &'static [PlaceIndex] = &[
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    fn adjacent(&self, x: Num, y: Num) -> u8 {
        let mut count = 0;
        for &(a, b) in Self::DIRECTIONS {
            if self.get(x + a, y + b) == Place::Occupied {
                count += 1;
            }
        }
        count
    }

    fn adjacent_skip_floor(&self, x: Num, y: Num) -> u8 {
        let mut count = 0;
        for &movement in Self::DIRECTIONS {
            if self.is_occupied_in_direction(x, y, movement) {
                count += 1;
            }
        }
        count
    }

    fn is_occupied_in_direction(&self, mut x: Num, mut y: Num, (mx, my): PlaceIndex) -> bool {
        loop {
            x += mx;
            y += my;
            match self.get(x, y) {
                Place::Empty | Place::Outside => break false,
                Place::Occupied => break true,
                Place::Floor => {}
            }
        }
    }

    fn count_occupied(&self) -> usize {
        self.places
            .iter()
            .filter(|&&place| place == Place::Occupied)
            .count()
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Seats {
    let width = input.lines().next().unwrap().len() + 2;

    let border = std::iter::repeat(Place::Outside).take(width + 1);
    let places: Vec<Place> = border
        .clone()
        .chain(input.bytes().flat_map(char_to_places))
        .chain(border)
        .collect();

    let height = places.len() / width;

    Seats::new(places, width as Num, height as Num)
}

fn char_to_places(c: u8) -> impl Iterator<Item = Place> {
    let slice = match c {
        b'.' => &[Some(Place::Floor), None],
        b'L' => &[Some(Place::Empty), None],
        b'#' => &[Some(Place::Occupied), None],
        b'\n' => &[Some(Place::Outside), Some(Place::Outside)],
        _ => &[None, None],
    };
    slice.iter().copied().flatten()
}

#[aoc(day11, part1)]
fn part1(seats: &Seats) -> usize {
    part_impl(seats, true)
}

#[aoc(day11, part2)]
fn part2(seats: &Seats) -> usize {
    part_impl(seats, false)
}

fn part_impl(seats: &Seats, part1: bool) -> usize {
    let mut seats = seats.clone();
    let mut queue = Vec::new();

    loop {
        queue.clear();

        for x in 1..seats.width - 1 {
            for y in 1..seats.height - 1 {
                let place = seats.get(x, y);

                if place != Place::Floor {
                    if part1 {
                        let adjacent = seats.adjacent(x, y);
                        if (place == Place::Empty && adjacent == 0)
                            || (place == Place::Occupied && adjacent >= 4)
                        {
                            queue.push((x, y));
                        }
                    } else {
                        let adjacent = seats.adjacent_skip_floor(x, y);
                        if (place == Place::Empty && adjacent == 0)
                            || (place == Place::Occupied && adjacent >= 5)
                        {
                            queue.push((x, y));
                        }
                    }
                }
            }
        }

        if queue.is_empty() {
            break seats.count_occupied();
        }

        for &(x, y) in &queue {
            let place = seats.get_mut(x, y);
            match place {
                Place::Empty => *place = Place::Occupied,
                Place::Occupied => *place = Place::Empty,
                _ => {}
            }
        }
    }
}
