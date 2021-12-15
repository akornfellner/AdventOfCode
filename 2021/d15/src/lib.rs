use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs};

pub fn solve(filename: &str, p2: bool) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in lines {
        let mut l: Vec<usize> = vec![];
        for c in line.chars().collect::<Vec<char>>() {
            l.push(c as usize - 48);
        }
        numbers.push(l);
    }

    if p2 {
        numbers = expand_field(&numbers);
    }

    let rows = numbers.len() - 1;
    let cols = numbers[0].len() - 1;

    let mut risks: HashMap<Pos, usize> = HashMap::new();

    for i in 0..=rows {
        for j in 0..=cols {
            risks.insert(Pos(i, j), numbers[i][j]);
        }
    }

    let end = Pos(rows, cols);

    let result = dijkstra(
        &Pos(0, 0),
        |p| p.neighbours(&risks, rows, cols),
        |p| *p == end,
    );

    result.unwrap().1
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(
        &self,
        risks: &HashMap<Pos, usize>,
        rows: usize,
        cols: usize,
    ) -> Vec<(Pos, usize)> {
        let mut result: Vec<(Pos, usize)> = vec![];

        let &Pos(i, j) = self;

        if self.0 > 0 {
            result.push((Pos(i - 1, j), risks[&Pos(i - 1, j)]));
        }

        if self.1 > 0 {
            result.push((Pos(i, j - 1), risks[&Pos(i, j - 1)]));
        }

        if self.0 < rows {
            result.push((Pos(i + 1, j), risks[&Pos(i + 1, j)]));
        }

        if self.1 < cols {
            result.push((Pos(i, j + 1), risks[&Pos(i, j + 1)]));
        }

        result
    }
}

pub fn expand_field(numbers: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut expand: Vec<Vec<usize>> = vec![];

    for line in numbers {
        let mut l: Vec<usize> = vec![];
        for i in 0..5 {
            for number in line {
                l.push(reduce(number + i));
            }
        }
        expand.push(l);
    }

    let tmp = expand.clone();
    expand.clear();

    for i in 0..5 {
        for line in &tmp {
            let new: Vec<usize> = line.iter().map(|x| reduce(*x + i)).collect();
            expand.push(new);
        }
    }

    expand
}

fn reduce(mut x: usize) -> usize {
    if x > 9 {
        x -= 9;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn p1_works() {
        assert_eq!(solve("input_test.txt", false), 40);
    }

    #[test]
    fn p2_works() {
        assert_eq!(solve("input_test.txt", true), 315);
    }
}
