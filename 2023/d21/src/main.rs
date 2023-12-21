use std::collections::{HashMap, HashSet};

fn main() {
    let (p1, p2) = solve("input_test.in", 50);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str, steps: i64) -> (i64, i64) {
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

    let mut transformations: Transformations = HashMap::new();

    let mut queue = HashSet::new();
    queue.insert(start);

    for _ in 0..steps {
        let mut new_queue = HashSet::new();

        for pos in queue {
            let neighbors = get_neighbors(pos, &gardens, &mut transformations);

            for neighbor in neighbors {
                new_queue.insert(neighbor);
            }
        }

        queue = new_queue;
    }

    result.0 += queue.len() as i64;

    result
}

type Transformations = HashMap<(i64, i64), Vec<(i64, i64)>>;

fn get_neighbors(
    pos: (i64, i64),
    gardens: &[Vec<Garden>],
    transformations: &mut Transformations,
) -> Vec<(i64, i64)> {
    let mut neighbors = vec![];
    let (x, y) = pos;

    let (i, j, dx, dy) = transform(x, y, gardens);
    if transformations.contains_key(&(i, j)) {
        for neighbor in transformations.get(&(i, j)).unwrap() {
            neighbors.push((neighbor.0 + dx, neighbor.1 + dy));
        }
        return neighbors;
    }

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

    if (x >= 0 && x < gardens.len() as i64 && y >= 0 && y < gardens[0].len() as i64)
        && !transformations.contains_key(&pos)
    {
        transformations.insert(pos, neighbors.clone());
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
