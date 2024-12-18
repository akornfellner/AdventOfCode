use pathfinding::prelude::bfs;
use std::env::args;
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, String) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let width = 71;
    let height = 71;
    let bytes = 1024;

    let mut grid = vec![vec![true; width]; height];

    let mut coords = vec![];

    for line in input.lines() {
        let parts = line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        let (x, y) = (parts[0], parts[1]);

        coords.push((x, y));
    }

    for (x, y) in coords.iter().take(bytes) {
        grid[*y][*x] = false;
    }

    let mut path = get_path(&grid).unwrap();
    let p1 = path.len() - 1;
    let mut p2 = String::new();

    for (x, y) in coords.iter().skip(bytes) {
        grid[*y][*x] = false;
        if !path.contains(&(*x, *y)) {
            continue;
        }
        match get_path(&grid) {
            Some(new_path) => {
                if new_path.len() > path.len() {
                    path = new_path;
                }
            }
            None => {
                p2 = format!("{},{}", x, y);
                break;
            }
        }
    }

    (p1, p2)
}

fn successors((x, y): (usize, usize), grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if x > 0 && grid[y][x - 1] {
        result.push((x - 1, y));
    }

    if x < grid[0].len() - 1 && grid[y][x + 1] {
        result.push((x + 1, y));
    }

    if y > 0 && grid[y - 1][x] {
        result.push((x, y - 1));
    }

    if y < grid.len() - 1 && grid[y + 1][x] {
        result.push((x, y + 1));
    }

    result
}

fn get_path(grid: &[Vec<bool>]) -> Option<Vec<(usize, usize)>> {
    bfs(
        &(0, 0),
        |&pos| successors(pos, grid),
        |&pos| pos == (grid[0].len() - 1, grid.len() - 1),
    )
}
