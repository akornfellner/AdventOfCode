use std::env::args;

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

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut field1 = vec![];
    let mut field2 = vec![];
    let mut robot1 = (0, 0);

    for (y, line) in parts[0].lines().enumerate() {
        let mut row1 = vec![];
        let mut row2 = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                robot1 = (x, y);
            }
            let add = match c {
                'O' => ('[', ']'),
                '@' => ('@', '.'),
                '#' => ('#', '#'),
                _ => ('.', '.'),
            };
            row1.push(c);
            row2.push(add.0);
            row2.push(add.1);
        }
        field1.push(row1);
        field2.push(row2);
    }

    let mut robot2 = (robot1.0 * 2, robot1.1);

    let moves = parts[1].replace("\n", "");
    let moves = moves.chars().collect::<Vec<char>>();

    for m in &moves {
        if make_move(robot1, *m, &mut field1) {
            robot1 = next_pos(robot1, *m);
        }
    }

    for m in &moves {
        if make_move(robot2, *m, &mut field2) {
            robot2 = next_pos(robot2, *m);
        }
    }

    (checksum(&field1, false), checksum(&field2, true))
}

fn make_move((x, y): (usize, usize), dir: char, field: &mut Vec<Vec<char>>) -> bool {
    let (prev_x, prev_y) = prev_pos((x, y), dir);

    match field[y][x] {
        '#' => false,
        'O' | '[' | ']' => {
            if field[y][x] == 'O' {
                if make_move(next_pos((x, y), dir), dir, field) {
                    field[y][x] = field[prev_y][prev_x];
                    field[prev_y][prev_x] = '.';
                    true
                } else {
                    false
                }
            } else {
                match dir {
                    '^' | 'v' => {
                        if is_possible((x, y), dir, field) {
                            let side = if field[y][x] == '[' { '>' } else { '<' };
                            let side = next_pos((x, y), side);
                            make_move(next_pos((x, y), dir), dir, field);
                            make_move(next_pos(side, dir), dir, field);
                            field[y][x] = field[prev_y][prev_x];
                            field[prev_y][prev_x] = '.';

                            true
                        } else {
                            false
                        }
                    }
                    _ => {
                        if make_move(next_pos((x, y), dir), dir, field) {
                            field[y][x] = field[prev_y][prev_x];
                            field[prev_y][prev_x] = '.';
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
        '@' => {
            if make_move(next_pos((x, y), dir), dir, field) {
                field[y][x] = '.';
                true
            } else {
                false
            }
        }
        _ => {
            field[y][x] = field[prev_y][prev_x];
            field[prev_y][prev_x] = '.';
            true
        }
    }
}

fn next_pos((x, y): (usize, usize), dir: char) -> (usize, usize) {
    match dir {
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        '<' => (x - 1, y),
        _ => (x + 1, y),
    }
}

fn prev_pos((x, y): (usize, usize), dir: char) -> (usize, usize) {
    match dir {
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        '<' => (x + 1, y),
        _ => (x - 1, y),
    }
}

fn is_possible((x, y): (usize, usize), dir: char, field: &[Vec<char>]) -> bool {
    match field[y][x] {
        '#' => false,
        '[' => {
            let next = next_pos((x, y), dir);
            let right = next_pos(next, '>');
            is_possible(next, dir, field) && is_possible(right, dir, field)
        }
        ']' => {
            let next = next_pos((x, y), dir);
            let left = next_pos(next, '<');
            is_possible(next, dir, field) && is_possible(left, dir, field)
        }
        _ => true,
    }
}

fn checksum(field: &[Vec<char>], two: bool) -> usize {
    let compare = if two { '[' } else { 'O' };
    let mut result = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == compare {
                result += y * 100 + x;
            }
        }
    }
    result
}
