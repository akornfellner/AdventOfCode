use std::{collections::HashSet, time};

fn main() {
    let start = time::Instant::now();
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
    println!("Duration: {:?}", start.elapsed());
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let cmds: Vec<Cmd> = input.lines().map(Cmd::from).collect();

    result.0 = get_cubic_meters(&cmds);

    let cmds2: Vec<Cmd> = input.lines().map(Cmd::from_hex).collect();

    result
}

fn get_cubic_meters(cmds: &[Cmd]) -> usize {
    let mut current = (0, 0);

    let mut bounds: HashSet<(i64, i64)> = HashSet::new();
    bounds.insert(current);

    let mut inside: HashSet<(i64, i64)> = HashSet::new();

    let mut last_direction = Direction::Down;
    let mut leftcount = 0;
    let mut rightcount = 0;

    for cmd in cmds {
        match cmd.direction {
            Direction::Up => match last_direction {
                Direction::Left => rightcount += 1,
                Direction::Right => leftcount += 1,
                _ => {}
            },
            Direction::Down => match last_direction {
                Direction::Left => leftcount += 1,
                Direction::Right => rightcount += 1,
                _ => {}
            },
            Direction::Left => match last_direction {
                Direction::Up => leftcount += 1,
                Direction::Down => rightcount += 1,
                _ => {}
            },
            Direction::Right => match last_direction {
                Direction::Up => rightcount += 1,
                Direction::Down => leftcount += 1,
                _ => {}
            },
        }
        last_direction = cmd.direction;
    }

    let clockwise = if rightcount > leftcount { true } else { false };

    for cmd in cmds {
        for _ in 0..cmd.distance {
            match cmd.direction {
                Direction::Up => {
                    current.0 -= 1;
                    if clockwise {
                        inside.insert((current.0 + 1, current.1 + 1));
                        inside.insert((current.0, current.1 + 1));
                    } else {
                        inside.insert((current.0 + 1, current.1 - 1));
                        inside.insert((current.0, current.1 - 1));
                    }
                }
                Direction::Down => {
                    current.0 += 1;
                    if clockwise {
                        inside.insert((current.0 - 1, current.1 - 1));
                        inside.insert((current.0, current.1 - 1));
                    } else {
                        inside.insert((current.0 - 1, current.1 + 1));
                        inside.insert((current.0, current.1 + 1));
                    }
                }
                Direction::Left => {
                    current.1 -= 1;
                    if clockwise {
                        inside.insert((current.0 - 1, current.1 + 1));
                        inside.insert((current.0 - 1, current.1));
                    } else {
                        inside.insert((current.0 + 1, current.1 + 1));
                        inside.insert((current.0 + 1, current.1));
                    }
                }
                Direction::Right => {
                    current.1 += 1;
                    if clockwise {
                        inside.insert((current.0 + 1, current.1 - 1));
                        inside.insert((current.0 + 1, current.1));
                    } else {
                        inside.insert((current.0 - 1, current.1 - 1));
                        inside.insert((current.0 - 1, current.1));
                    }
                }
            }
            bounds.insert(current);
        }
    }

    let mut inside: HashSet<(i64, i64)> = inside.difference(&bounds).cloned().collect();
    let mut rest: Vec<(i64, i64)> = inside.iter().map(|x| (x.0, x.1)).collect();

    while !rest.is_empty() {
        let current = rest.pop().unwrap();
        inside.insert(current);
        for (x, y) in [
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ] {
            if !bounds.contains(&(x, y)) && !inside.contains(&(x, y)) {
                rest.push((x, y));
            }
        }
    }

    inside.len() + bounds.len()
}

#[derive(Debug)]
struct Cmd {
    direction: Direction,
    distance: i64,
}

impl Cmd {
    fn from(s: &str) -> Cmd {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let direction = Direction::from_char(parts[0].chars().next().unwrap());
        let distance = parts[1].parse().unwrap();

        Cmd {
            direction,
            distance,
        }
    }

    fn from_hex(hex: &str) -> Cmd {
        let hex = hex.split_whitespace().collect::<Vec<&str>>()[2].replace(['(', ')', '#'], "");
        let distance = i64::from_str_radix(&hex[..5], 16).unwrap();
        let direction = match hex.chars().nth(5).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            _ => Direction::Up,
        };

        Cmd {
            direction,
            distance,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
