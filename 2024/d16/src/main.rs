use pathfinding::prelude::yen;
use std::{collections::HashSet, env::args};

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

    let field: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in field.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
            } else if *c == 'E' {
                end = (x, y);
            }
        }
    }

    let result = yen(
        &(start, Direction::Right),
        |&(pos, dir)| successors(pos, &mut dir.clone(), &field),
        |&(pos, _)| pos == end,
        15, // if your solution is not working, try to increase this value!!!
    );

    let min_costs = result[0].1;

    let mut tiles = HashSet::new();

    for (path, costs) in result {
        if costs == min_costs {
            for (pos, _) in path {
                tiles.insert(pos);
            }
        }
    }

    (min_costs, tiles.len())
}

type State = ((usize, usize), Direction);

fn successors(
    (x, y): (usize, usize),
    d: &mut Direction,
    field: &[Vec<char>],
) -> Vec<(State, usize)> {
    let mut result = vec![];
    let mut tmp = vec![];
    match d {
        Direction::Right => {
            tmp.push((((x + 1, y), Direction::Right), 1));
            tmp.push((((x, y + 1), Direction::Down), 1001));
            tmp.push((((x, y - 1), Direction::Up), 1001));
        }
        Direction::Left => {
            tmp.push((((x - 1, y), Direction::Left), 1));
            tmp.push((((x, y + 1), Direction::Down), 1001));
            tmp.push((((x, y - 1), Direction::Up), 1001));
        }
        Direction::Up => {
            tmp.push((((x, y - 1), Direction::Up), 1));
            tmp.push((((x + 1, y), Direction::Right), 1001));
            tmp.push((((x - 1, y), Direction::Left), 1001));
        }
        Direction::Down => {
            tmp.push((((x, y + 1), Direction::Down), 1));
            tmp.push((((x + 1, y), Direction::Right), 1001));
            tmp.push((((x - 1, y), Direction::Left), 1001));
        }
    }

    for (((x, y), new_dir), c) in tmp {
        if x < field[0].len() && y < field.len() && field[y][x] != '#' {
            result.push((((x, y), new_dir), c));
        }
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
