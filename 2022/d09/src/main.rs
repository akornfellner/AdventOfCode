use std::{collections::HashSet, fs};

fn main() {
    println!("part one: {}", solve("input.txt", 2));
    println!("part two: {}", solve("input.txt", 10));
}

fn solve(input: &str, knots: usize) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let moves: Vec<(&str, i32)> = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
        .collect();

    let mut rope = Rope::new(knots);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for m in &moves {
        let direction = m.0;
        let n = m.1;

        for _ in 0..n {
            match direction {
                "R" => rope.add_head(0, 1),
                "L" => rope.add_head(0, -1),
                "U" => rope.add_head(-1, 0),
                _ => rope.add_head(1, 0),
            }

            for i in 0..rope.knots.len() - 1 {
                let neighbors = neighbors(rope.knots[i + 1]);
                let mut m = distance(rope.knots[i], rope.knots[i + 1]);

                if m >= 1.5 {
                    for neighbor in &neighbors {
                        let new = distance(rope.knots[i], *neighbor);
                        if new < m {
                            rope.knots[i + 1] = *neighbor;
                            m = new;
                        }
                    }
                }
            }
            visited.insert(rope.get_last());
        }
    }

    visited.len() as i32
}

fn distance(a: (i32, i32), b: (i32, i32)) -> f64 {
    f64::sqrt(((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64)
}

fn neighbors(knot: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                result.push((knot.0 + i, knot.1 + j));
            }
        }
    }
    result
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Self {
            knots: vec![(0, 0); size],
        }
    }

    fn add_head(&mut self, x: i32, y: i32) {
        self.knots[0].0 += x;
        self.knots[0].1 += y;
    }

    fn get_last(&self) -> (i32, i32) {
        *self.knots.iter().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test_1.txt", 2);
        assert_eq!(result, 13);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test_2.txt", 10);
        assert_eq!(result, 36);
    }
}
