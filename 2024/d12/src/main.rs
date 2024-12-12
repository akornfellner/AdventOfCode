use std::collections::HashSet;

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
    let mut visited: Set = HashSet::new();

    let mut p1 = 0;
    let mut p2 = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            if !visited.contains(&(x, y)) {
                let mut upper_lines: Set = HashSet::new();
                let mut lower_lines: Set = HashSet::new();
                let mut left_lines: Set = HashSet::new();
                let mut right_lines: Set = HashSet::new();
                let area = find_region(
                    x,
                    y,
                    &field,
                    &mut visited,
                    &mut upper_lines,
                    &mut lower_lines,
                    &mut left_lines,
                    &mut right_lines,
                );
                let pers =
                    upper_lines.len() + lower_lines.len() + left_lines.len() + right_lines.len();
                let sides = get_sides(upper_lines)
                    + get_sides(lower_lines)
                    + get_sides(left_lines)
                    + get_sides(right_lines);
                p1 += area * pers;
                p2 += area * sides;
            }
        }
    }

    (p1, p2)
}

type Set = HashSet<(usize, usize)>;

fn find_region(
    x: usize,
    y: usize,
    field: &[Vec<char>],
    visited: &mut Set,
    upper_lines: &mut Set,
    lower_lines: &mut Set,
    left_lines: &mut Set,
    right_lines: &mut Set,
) -> usize {
    if visited.contains(&(x, y)) {
        return 0;
    }

    visited.insert((x, y));

    let neighbors = get_neighbors(x, y, field, visited);
    let perimeters = get_perimeters(x, y, field);
    let mut area = 1;

    if perimeters[0] == 1 {
        right_lines.insert((y + 1, x));
    }
    if perimeters[1] == 1 {
        lower_lines.insert((x + 1, y));
    }
    if perimeters[2] == 1 {
        left_lines.insert((y, x));
    }
    if perimeters[3] == 1 {
        upper_lines.insert((x, y));
    }

    for (nx, ny) in neighbors {
        let a = find_region(
            nx,
            ny,
            field,
            visited,
            upper_lines,
            lower_lines,
            left_lines,
            right_lines,
        );

        area += a;
    }

    area
}

fn get_perimeters(x: usize, y: usize, field: &[Vec<char>]) -> [usize; 4] {
    let mut perimeters = [0, 0, 0, 0];
    let possible = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let x = x as i32;
    let y = y as i32;

    for (i, (dx, dy)) in possible.iter().enumerate() {
        let nx = x + dx;
        let ny = y + dy;

        if !(nx >= 0
            && nx < field.len() as i32
            && ny >= 0
            && ny < field[0].len() as i32
            && field[nx as usize][ny as usize] == field[x as usize][y as usize])
        {
            perimeters[i] = 1;
        }
    }

    perimeters
}

fn get_neighbors(
    x: usize,
    y: usize,
    field: &[Vec<char>],
    visited: &mut Set,
) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let possible = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let x = x as i32;
    let y = y as i32;

    for (dx, dy) in &possible {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0
            && nx < field.len() as i32
            && ny >= 0
            && ny < field[0].len() as i32
            && field[nx as usize][ny as usize] == field[x as usize][y as usize]
            && !visited.contains(&(nx as usize, ny as usize))
        {
            neighbors.push((nx as usize, ny as usize));
        }
    }

    neighbors
}

fn get_sides(lines: Set) -> usize {
    let mut sides = 1;

    let mut lines: Vec<(usize, usize)> = lines.into_iter().collect();
    lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    for i in 1..lines.len() {
        if lines[i].0 != lines[i - 1].0 || lines[i].1 - lines[i - 1].1 > 1 {
            sides += 1;
        }
    }

    sides
}
