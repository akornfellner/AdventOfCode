use std::{
    collections::{HashMap, HashSet},
    env::args,
};
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut field = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (x, y);
                    row.push('.');
                }
                'E' => {
                    end = (x, y);
                    row.push('.');
                }
                _ => row.push(c),
            }
        }
        field.push(row);
    }

    let mut path: Path = HashMap::new();
    path.insert(start, 0);

    let mut last = start;
    let mut current = start;

    let mut steps = 0;

    while current != end {
        let tmp = current;
        current = get_next(current, last, &field);
        last = tmp;
        steps += 1;
        path.insert(current, steps);
    }

    let mut cheats = vec![];

    for ((x, y), _) in &path {
        for cheat in cheat((*x, *y), &field, &path) {
            cheats.push(cheat);
        }
    }

    let p1 = cheats.iter().filter(|x| **x >= 100).count();

    (p1, 0)
}

fn cheat((x, y): (usize, usize), field: &[Vec<char>], path: &Path) -> Vec<usize> {
    let mut result = vec![];
    for neighbor in get_neighbors((x, y), field, '#') {
        for neighbor2 in get_neighbors(neighbor, field, '.') {
            if let Some(steps) = path.get(&neighbor2) {
                if *steps > path[&(x, y)] + 2 {
                    result.push(*steps - path[&(x, y)] - 2);
                }
            }
        }
    }
    result
}

fn get_next((x, y): (usize, usize), last: (usize, usize), field: &[Vec<char>]) -> (usize, usize) {
    let mut next = (0, 0);
    for neighbor in get_neighbors((x, y), field, '.') {
        if neighbor != last {
            next = neighbor;
            break;
        }
    }
    next
}

type Path = HashMap<(usize, usize), usize>;

fn get_neighbors((x, y): (usize, usize), field: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 && field[y][x - 1] == c {
        neighbors.push((x - 1, y));
    }
    if x < field[0].len() - 1 && field[y][x + 1] == c {
        neighbors.push((x + 1, y));
    }
    if y > 0 && field[y - 1][x] == c {
        neighbors.push((x, y - 1));
    }
    if y < field.len() - 1 && field[y + 1][x] == c {
        neighbors.push((x, y + 1));
    }
    neighbors
}
