fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let cmds: Vec<Cmd> = input.lines().map(Cmd::from).collect();
    let cmds2: Vec<Cmd> = input.lines().map(Cmd::from_hex).collect();

    result.0 = get_area(&cmds);
    result.1 = get_area(&cmds2);

    result
}

fn get_area(cmds: &[Cmd]) -> i64 {
    let corners = get_corners(cmds);

    let mut bound = 0i64;
    let mut area = 0i64;

    for i in 0..corners.len() - 1 {
        let (x1, y1) = corners[i];
        let (x2, y2) = corners[i + 1];

        area += x1 * y2 - x2 * y1;
        bound += (x1 - x2).abs() + (y1 - y2).abs();
    }

    area = area.abs() / 2;

    area + bound / 2 + 1
}

fn get_corners(cmds: &[Cmd]) -> Vec<(i64, i64)> {
    let mut current = (0, 0);
    let mut corners = vec![current];

    for cmd in cmds {
        match cmd.direction {
            Direction::Up => current.0 -= cmd.distance,
            Direction::Down => current.0 += cmd.distance,
            Direction::Left => current.1 -= cmd.distance,
            Direction::Right => current.1 += cmd.distance,
        }
        corners.push(current);
    }

    corners
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
