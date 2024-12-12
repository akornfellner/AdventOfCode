use std::collections::HashSet;

use stopwatch::time;

#[time]
fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let field: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = HashSet::new();

    let mut p1 = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            if !visited.contains(&(x, y)) {
                let (area, perimeter) = find_region(x, y, &field, &mut visited);
                p1 += area * perimeter;
            }
        }
    }

    (p1, 0)
}

fn find_region(
    x: usize,
    y: usize,
    field: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    if visited.contains(&(x, y)) {
        return (0, 0);
    }

    visited.insert((x, y));

    let (neighbors, p) = get_neighbors(x, y, field, visited);
    let mut perimeter = p;
    let mut area = 1;

    for (nx, ny) in neighbors {
        let (a, p) = find_region(nx, ny, field, visited);
        area += a;
        perimeter += p;
    }

    (area, perimeter)
}

fn get_neighbors(
    x: usize,
    y: usize,
    field: &[Vec<char>],
    visited: &HashSet<(usize, usize)>,
) -> (Vec<(usize, usize)>, usize) {
    let mut neighbors = vec![];
    let possible = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let x = x as i32;
    let y = y as i32;
    let mut perimeter = 0;

    for (dx, dy) in possible.iter() {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0
            && nx < field.len() as i32
            && ny >= 0
            && ny < field[0].len() as i32
            && field[nx as usize][ny as usize] == field[x as usize][y as usize]
        {
            if !visited.contains(&(nx as usize, ny as usize)) {
                neighbors.push((nx as usize, ny as usize));
            }
        } else {
            perimeter += 1;
        }
    }

    (neighbors, perimeter)
}
