use std::{collections::HashMap, env::args};

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

    let p: Vec<(usize, usize)> = path.keys().cloned().collect();

    let mut p1 = 0;
    let mut p2 = 0;

    for (i, (x1, y1)) in p.iter().enumerate() {
        for (x2, y2) in p.iter().skip(i + 1) {
            let d = (*x1 as isize - *x2 as isize).abs() + (*y1 as isize - *y2 as isize).abs();
            let time = ((path[&(*x1, *y1)] as isize) - (path[&(*x2, *y2)] as isize)).abs() - d;
            if time >= 100 {
                if d == 2 {
                    p1 += 1;
                } else if d <= 20 {
                    p2 += 1;
                }
            }
        }
    }

    p2 += p1;

    (p1, p2)
}

fn get_next((x, y): (usize, usize), last: (usize, usize), field: &[Vec<char>]) -> (usize, usize) {
    let mut next = (0, 0);
    for neighbor in get_neighbors((x, y), field) {
        if neighbor != last {
            next = neighbor;
            break;
        }
    }
    next
}

type Path = HashMap<(usize, usize), usize>;

fn get_neighbors((x, y): (usize, usize), field: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 && field[y][x - 1] == '.' {
        neighbors.push((x - 1, y));
    }
    if x < field[0].len() - 1 && field[y][x + 1] == '.' {
        neighbors.push((x + 1, y));
    }
    if y > 0 && field[y - 1][x] == '.' {
        neighbors.push((x, y - 1));
    }
    if y < field.len() - 1 && field[y + 1][x] == '.' {
        neighbors.push((x, y + 1));
    }
    neighbors
}
