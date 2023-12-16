use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let mut field: Field = vec![];

    for line in input.lines() {
        let mut row: Vec<(char, bool)> = vec![];
        for c in line.chars() {
            row.push((c, false));
        }
        field.push(row);
    }

    result.0 = walk(
        (0, 0),
        &mut field.clone(),
        Direction::Right,
        &mut HashSet::new(),
    );

    for i in 0..field.len() {
        result.1 = result.1.max(walk(
            (i, 0),
            &mut field.clone(),
            Direction::Right,
            &mut HashSet::new(),
        ));
        result.1 = result.1.max(walk(
            (i, field[0].len() - 1),
            &mut field.clone(),
            Direction::Left,
            &mut HashSet::new(),
        ));
    }

    for i in 0..field[0].len() {
        result.1 = result.1.max(walk(
            (0, i),
            &mut field.clone(),
            Direction::Down,
            &mut HashSet::new(),
        ));
        result.1 = result.1.max(walk(
            (field.len() - 1, i),
            &mut field.clone(),
            Direction::Up,
            &mut HashSet::new(),
        ));
    }

    result
}

type Visisted = HashSet<((usize, usize), Direction)>;
type Field = Vec<Vec<(char, bool)>>;

fn walk(
    start: (usize, usize),
    field: &mut [Vec<(char, bool)>],
    direction: Direction,
    visited: &mut Visisted,
) -> usize {
    let mut result = if field[start.0][start.1].1 { 0 } else { 1 };
    visited.insert((start, direction));
    let mut current = start;
    field[current.0][current.1].1 = true;

    match field[current.0][current.1].0 {
        '\\' => {
            let direction = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            if step(&mut current, &direction, field) && !visited.contains(&(current, direction)) {
                result += walk(current, field, direction, visited);
            }
        }
        '/' => {
            let direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };

            if step(&mut current, &direction, field) && !visited.contains(&(current, direction)) {
                result += walk(current, field, direction, visited);
            }
        }
        '|' => match direction {
            Direction::Up | Direction::Down => {
                if step(&mut current, &direction, field) && !visited.contains(&(current, direction))
                {
                    result += walk(current, field, direction, visited);
                }
            }
            Direction::Left | Direction::Right => {
                let mut up = current;
                let mut down = current;

                if step(&mut up, &Direction::Up, field) && !visited.contains(&(up, Direction::Up)) {
                    result += walk(up, field, Direction::Up, visited);
                }
                if step(&mut down, &Direction::Down, field)
                    && !visited.contains(&(down, Direction::Down))
                {
                    result += walk(down, field, Direction::Down, visited);
                }
            }
        },
        '-' => match direction {
            Direction::Left | Direction::Right => {
                if step(&mut current, &direction, field) && !visited.contains(&(current, direction))
                {
                    result += walk(current, field, direction, visited);
                }
            }
            Direction::Up | Direction::Down => {
                let mut left = current;
                let mut right = current;

                if step(&mut left, &Direction::Left, field)
                    && !visited.contains(&(left, Direction::Left))
                {
                    result += walk(left, field, Direction::Left, visited);
                }
                if step(&mut right, &Direction::Right, field)
                    && !visited.contains(&(right, Direction::Right))
                {
                    result += walk(right, field, Direction::Right, visited);
                }
            }
        },
        _ => {
            if step(&mut current, &direction, field) && !visited.contains(&(current, direction)) {
                result += walk(current, field, direction, visited);
            }
        }
    }
    result
}

fn step(
    current: &mut (usize, usize),
    direction: &Direction,
    field: &mut [Vec<(char, bool)>],
) -> bool {
    match direction {
        Direction::Up => {
            if current.0 > 0 {
                current.0 -= 1;
                true
            } else {
                false
            }
        }
        Direction::Down => {
            if current.0 < field.len() - 1 {
                current.0 += 1;
                true
            } else {
                false
            }
        }
        Direction::Left => {
            if current.1 > 0 {
                current.1 -= 1;
                true
            } else {
                false
            }
        }
        Direction::Right => {
            if current.1 < field[0].len() - 1 {
                current.1 += 1;
                true
            } else {
                false
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
