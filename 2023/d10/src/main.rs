use std::collections::{HashMap, HashSet};

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (1, 0);

    let mut tiles: Tiles = HashMap::new();
    tiles.insert('.', ((0, 0), (0, 0)));
    tiles.insert('|', ((-1, 0), (1, 0)));
    tiles.insert('-', ((0, -1), (0, 1)));
    tiles.insert('L', ((-1, 0), (0, 1)));
    tiles.insert('J', ((-1, 0), (0, -1)));
    tiles.insert('7', ((0, -1), (1, 0)));
    tiles.insert('F', ((1, 0), (0, 1)));
    tiles.insert('S', ((0, 0), (0, 0)));

    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut start = Pos::new();

    'outer: for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                (start.x, start.y) = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    let mut prev = start.clone();
    let mut pos = start.get_valid_neighbor(&map, &tiles).clone();

    let mut loop_tiles: HashSet<Pos> = HashSet::new();
    loop_tiles.insert(start.clone());
    let mut left_count = 0;
    let mut right_count = 0;

    loop {
        loop_tiles.insert(pos.clone());
        match pos.step(&mut prev, &map, &tiles) {
            Turn::Left => left_count += 1,
            Turn::Right => right_count += 1,
            _ => (),
        }
        result.0 += 1;
        if map[pos.x as usize][pos.y as usize] == 'S' {
            break;
        }
    }

    result.0 /= 2;

    let clockwise = left_count < right_count;

    let mut prev = start.clone();
    let mut pos = start.get_valid_neighbor(&map, &tiles);
    let mut inclusions: HashSet<Pos> = HashSet::new();
    let mut direction;

    loop {
        direction = Direction::get(&prev, &pos);
        let n1 = prev.get_inside_neighbor(&direction, clockwise, &map);
        let n2 = pos.get_inside_neighbor(&direction, clockwise, &map);
        if !loop_tiles.contains(&n1) {
            inclusions.insert(n1.clone());
        }
        if !loop_tiles.contains(&n2) {
            inclusions.insert(n2.clone());
        }

        if map[pos.x as usize][pos.y as usize] == 'S' {
            break;
        }

        pos.step(&mut prev, &map, &tiles);
    }

    let mut rest: Vec<Pos> = inclusions.iter().cloned().collect();

    while !rest.is_empty() {
        let mut new_rest: Vec<Pos> = vec![];
        for pos in rest {
            let neighbors = pos.get_neighbors(&map);
            for neighbor in neighbors {
                if !inclusions.contains(&neighbor) && !loop_tiles.contains(&neighbor) {
                    inclusions.insert(neighbor.clone());
                    new_rest.push(neighbor.clone());
                }
            }
        }
        rest = new_rest;
    }

    result.1 = inclusions.len() as i32;

    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

type Tiles = HashMap<char, ((i32, i32), (i32, i32))>;

impl Pos {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn step(&mut self, prev: &mut Self, map: &[Vec<char>], tiles: &Tiles) -> Turn {
        let mut new = self.clone();
        let tile = map[self.x as usize][self.y as usize];
        let (a, b) = *tiles.get(&tile).unwrap();
        if prev.x - self.x == a.0 && prev.y - self.y == a.1 {
            new.x += b.0;
            new.y += b.1;
        } else {
            new.x += a.0;
            new.y += a.1;
        }

        let diff = (new.x - prev.x, new.y - prev.y);
        let direction = match tile {
            'L' => match diff {
                (1, 1) => Turn::Left,
                (-1, -1) => Turn::Right,
                _ => Turn::None,
            },
            'J' => match diff {
                (1, -1) => Turn::Right,
                (-1, 1) => Turn::Left,
                _ => Turn::None,
            },
            '7' => match diff {
                (-1, -1) => Turn::Left,
                (1, 1) => Turn::Right,
                _ => Turn::None,
            },
            'F' => match diff {
                (-1, 1) => Turn::Right,
                (1, -1) => Turn::Left,
                _ => Turn::None,
            },
            _ => Turn::None,
        };

        prev.x = self.x;
        prev.y = self.y;
        self.x = new.x;
        self.y = new.y;

        direction
    }

    fn get_inside_neighbor(
        &self,
        direction: &Direction,
        clockwise: bool,
        map: &[Vec<char>],
    ) -> Self {
        match direction {
            Direction::Up => {
                if clockwise {
                    if self.y < map[0].len() as i32 {
                        Self::from(self.x, self.y + 1)
                    } else {
                        Self::from(self.x, self.y)
                    }
                } else if self.y > 0 {
                    Self::from(self.x, self.y - 1)
                } else {
                    Self::from(self.x, self.y)
                }
            }
            Direction::Down => {
                if clockwise {
                    if self.y > 0 {
                        Self::from(self.x, self.y - 1)
                    } else {
                        Self::from(self.x, self.y)
                    }
                } else if self.y < map[0].len() as i32 {
                    Self::from(self.x, self.y + 1)
                } else {
                    Self::from(self.x, self.y)
                }
            }
            Direction::Left => {
                if clockwise {
                    if self.x > 0 {
                        Self::from(self.x - 1, self.y)
                    } else {
                        Self::from(self.x, self.y)
                    }
                } else if self.x < map.len() as i32 {
                    Self::from(self.x + 1, self.y)
                } else {
                    Self::from(self.x, self.y)
                }
            }
            Direction::Right => {
                if clockwise {
                    if self.x < map.len() as i32 {
                        Self::from(self.x + 1, self.y)
                    } else {
                        Self::from(self.x, self.y)
                    }
                } else if self.x > 0 {
                    Self::from(self.x - 1, self.y)
                } else {
                    Self::from(self.x, self.y)
                }
            }
        }
    }

    fn get_neighbors(&self, map: &[Vec<char>]) -> Vec<Self> {
        let mut neighbors: Vec<Self> = vec![];
        let (row, col) = (self.x, self.y);
        let rows = map.len() as i32;
        let cols = map[0].len() as i32;

        for i in -1..=1 {
            for j in -1..=1 {
                if (i, j) == (0, 0) {
                    continue;
                }
                let new_row = row + i;
                let new_col = col + j;

                if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                    neighbors.push(Self::from(new_row, new_col));
                }
            }
        }

        neighbors
    }

    fn get_valid_neighbor(&self, map: &[Vec<char>], tiles: &Tiles) -> Self {
        let mut candidates: Vec<Self> = vec![];
        let (row, col) = (self.x, self.y);

        let rows = map.len() as i32;
        let cols = map[0].len() as i32;

        if row > 0 {
            candidates.push(Self::from(row - 1, col));
        }
        if row < rows {
            candidates.push(Self::from(row + 1, col));
        }
        if col > 0 {
            candidates.push(Self::from(row, col - 1));
        }
        if col < cols {
            candidates.push(Self::from(row, col + 1));
        }

        for c in candidates {
            let (a, b) = *tiles.get(&map[c.x as usize][c.y as usize]).unwrap();
            if c.x + a.0 == self.x && c.y + a.1 == self.y
                || c.x + b.0 == self.x && c.y + b.1 == self.y
            {
                return c;
            }
        }

        Pos::new()
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
    None,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get(pos1: &Pos, pos2: &Pos) -> Self {
        match (pos2.x - pos1.x, pos2.y - pos1.y) {
            (1, 0) => Self::Down,
            (-1, 0) => Self::Up,
            (0, 1) => Self::Right,
            (0, -1) => Self::Left,
            _ => panic!("Invalid direction"),
        }
    }
}
