use std::collections::{HashMap, HashSet};

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if c.is_alphanumeric() {
                antennas
                    .entry(*c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let mut antinodes1: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes2: HashSet<(isize, isize)> = HashSet::new();

    for (_, positions) in antennas.iter() {
        for (i, (x1, y1)) in positions.iter().enumerate() {
            for (x2, y2) in positions.iter().skip(i + 1) {
                let dx = x2 - x1;
                let dy = y2 - y1;

                if is_valid(&map, x2 + dx, y2 + dy) {
                    antinodes1.insert((x2 + dx, y2 + dy));
                }
                if is_valid(&map, x1 - dx, y1 - dy) {
                    antinodes1.insert((x1 - dx, y1 - dy));
                }

                antinodes2.insert((*x1, *y1));

                for m in [-1, 1] {
                    let mut x = *x1;
                    let mut y = *y1;
                    while is_valid(&map, x + dx * m, y + dy * m) {
                        x += dx * m;
                        y += dy * m;
                        antinodes2.insert((x, y));
                    }
                }
            }
        }
    }

    (antinodes1.len(), antinodes2.len())
}

fn is_valid(map: &[Vec<char>], x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && x < map.len() as isize && y < map[0].len() as isize
}
