use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut gardens: Vec<Vec<Garden>> = vec![];
    let mut start = (0, 0);

    for (x, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (y, garden) in line.chars().enumerate() {
            let garden = match garden {
                '#' => Garden::Rock,
                '.' => Garden::Plot,
                _ => {
                    start = (x as i64, y as i64);
                    Garden::Plot
                }
            };
            row.push(garden);
        }
        gardens.push(row);
    }

    let size = gardens.len() as i64;

    let mut queue = HashSet::new();
    queue.insert(start);
    let mut sols = vec![];

    for i in 1..=65 + 2 * size {
        let mut new_queue = HashSet::new();

        for pos in queue {
            let neighbors = get_neighbors(pos, &gardens);

            for neighbor in neighbors {
                new_queue.insert(neighbor);
            }
        }

        queue = new_queue;

        if i == 64 {
            result.0 += queue.len() as i64;
        }

        if (i - 65) % size == 0 {
            sols.push(queue.len() as i64);
        }
    }

    let c = sols[0];
    let a = (sols[2] - 2 * sols[1] + c) / 2;
    let b = sols[1] - c - a;

    let x = (26501365 - 65) / size;

    result.1 = a * x * x + b * x + c;

    result
}

fn get_neighbors(pos: (i64, i64), gardens: &[Vec<Garden>]) -> Vec<(i64, i64)> {
    let mut neighbors = vec![];
    let (x, y) = pos;

    let (i, j, _, _) = transform(x - 1, y, gardens);
    if let Garden::Plot = gardens[i as usize][j as usize] {
        neighbors.push((x - 1, y));
    }

    let (i, j, _, _) = transform(x + 1, y, gardens);
    if let Garden::Plot = gardens[i as usize][j as usize] {
        neighbors.push((x + 1, y));
    }

    let (i, j, _, _) = transform(x, y - 1, gardens);
    if let Garden::Plot = gardens[i as usize][j as usize] {
        neighbors.push((x, y - 1));
    }

    let (i, j, _, _) = transform(x, y + 1, gardens);
    if let Garden::Plot = gardens[i as usize][j as usize] {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn transform(x: i64, y: i64, gardens: &[Vec<Garden>]) -> (i64, i64, i64, i64) {
    let mut i = x;
    let mut j = y;

    if i >= gardens.len() as i64 {
        i %= gardens.len() as i64;
    }

    if j >= gardens[0].len() as i64 {
        j %= gardens[0].len() as i64;
    }

    while i < 0 {
        i += gardens.len() as i64;
    }

    while j < 0 {
        j += gardens[0].len() as i64;
    }

    (i, j, x - i, y - j)
}

#[derive(Debug, Clone, Copy)]
enum Garden {
    Plot,
    Rock,
}
