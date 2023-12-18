use geo::{Area, EuclideanLength};
use geo_types::{Coord, LineString, Polygon};

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let cmds: Vec<Cmd> = input.lines().map(Cmd::from).collect();
    let cmds2: Vec<Cmd> = input.lines().map(Cmd::from_hex).collect();

    result.0 = get_area(&cmds);
    result.1 = get_area(&cmds2);

    result
}

fn get_area(cmds: &[Cmd]) -> usize {
    let corners = get_corners(cmds);

    let polygon = Polygon::new(corners.clone().into(), vec![]);
    let linestring = LineString::from(corners);

    let area = polygon.unsigned_area();
    let perimeter = linestring.euclidean_length();

    area.round() as usize + perimeter.round() as usize / 2 + 1
}

fn get_corners(cmds: &[Cmd]) -> Vec<Coord> {
    let mut current = (0.0, 0.0);
    let mut corners = vec![Coord::from(current)];

    for cmd in cmds {
        match cmd.direction {
            Direction::Up => current.0 -= cmd.distance as f64,
            Direction::Down => current.0 += cmd.distance as f64,
            Direction::Left => current.1 -= cmd.distance as f64,
            Direction::Right => current.1 += cmd.distance as f64,
        }
        corners.push(Coord::from(current));
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
