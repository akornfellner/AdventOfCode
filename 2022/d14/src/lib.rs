use std::{collections::HashMap, fs};

pub fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let rocks: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|d| d.split(',').collect::<Vec<&str>>())
                .map(|c| (c[0].parse::<i32>().unwrap(), c[1].parse::<i32>().unwrap()))
                .collect()
        })
        .collect();

    let mut cave = Cave::new();

    for r in &rocks {
        cave.add_rocks(r);
    }

    if two {
        cave.fill2();
    } else {
        cave.fill();
    }

    cave.count_sand()
}

#[derive(Debug)]
struct Cave {
    positions: HashMap<(i32, i32), Pos>,
    max: i32,
    start: (i32, i32),
}

impl Cave {
    fn new() -> Self {
        Self {
            positions: HashMap::from([((500, 0), Pos::Empty)]),
            max: 0,
            start: (500, 0),
        }
    }

    fn add_rocks(&mut self, rocks: &[(i32, i32)]) {
        for i in 0..rocks.len() - 1 {
            let mut a = rocks[i].0;
            let mut b = rocks[i + 1].0;

            if b < a {
                (a, b) = (b, a);
            }

            for x in a..=b {
                let mut a = rocks[i].1;
                let mut b = rocks[i + 1].1;

                if b < a {
                    (a, b) = (b, a);
                }

                for y in a..=b {
                    if y > self.max {
                        self.max = y;
                    }
                    self.positions.insert((x, y), Pos::Rock);
                }
            }
        }
    }

    fn fill(&mut self) {
        let mut finished = false;

        while !finished {
            let mut current = self.start;

            loop {
                if current.1 >= self.max {
                    finished = true;
                    break;
                } else if self.is_empty((current.0, current.1 + 1)) {
                    current = (current.0, current.1 + 1);
                } else if self.is_empty((current.0 - 1, current.1 + 1)) {
                    current = (current.0 - 1, current.1 + 1);
                } else if self.is_empty((current.0 + 1, current.1 + 1)) {
                    current = (current.0 + 1, current.1 + 1);
                } else {
                    self.positions.insert(current, Pos::Sand);
                    break;
                }
            }
        }
    }

    fn fill2(&mut self) {
        self.max += 2;
        let mut finished = false;

        while !finished {
            let mut current = self.start;

            loop {
                if current.1 + 1 >= self.max {
                    self.positions.insert(current, Pos::Sand);
                    break;
                } else if self.is_empty((current.0, current.1 + 1)) {
                    current = (current.0, current.1 + 1);
                } else if self.is_empty((current.0 - 1, current.1 + 1)) {
                    current = (current.0 - 1, current.1 + 1);
                } else if self.is_empty((current.0 + 1, current.1 + 1)) {
                    current = (current.0 + 1, current.1 + 1);
                } else {
                    self.positions.insert(current, Pos::Sand);
                    if current.1 <= 0 {
                        finished = true;
                    }
                    break;
                }
            }
        }
    }

    fn is_empty(&mut self, pos: (i32, i32)) -> bool {
        let new = self.positions.entry(pos).or_insert(Pos::Empty);
        matches!(new, Pos::Empty)
    }

    fn count_sand(&self) -> i32 {
        let mut count = 0;

        for pos in self.positions.values() {
            if let Pos::Sand = pos {
                count += 1;
            }
        }

        count
    }
}

#[derive(Debug, Clone)]
enum Pos {
    Sand,
    Rock,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 24);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 93);
    }
}
