use std::collections::HashMap;

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
    let mut pos = start.get_neighbors(&map, &tiles)[0].clone();

    loop {
        pos.step(&mut prev, &map, &tiles);
        result.0 += 1;
        if map[pos.x as usize][pos.y as usize] == 'S' {
            break;
        }
    }

    result.0 /= 2;

    result
}

#[derive(Debug, Clone)]
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

    fn step(&mut self, prev: &mut Self, map: &[Vec<char>], tiles: &Tiles) {
        let mut new = self.clone();
        let (a, b) = *tiles.get(&map[self.x as usize][self.y as usize]).unwrap();
        if prev.x - self.x == a.0 && prev.y - self.y == a.1 {
            new.x += b.0;
            new.y += b.1;
        } else {
            new.x += a.0;
            new.y += a.1;
        }
        prev.x = self.x;
        prev.y = self.y;
        self.x = new.x;
        self.y = new.y;
    }

    fn get_neighbors(&self, map: &[Vec<char>], tiles: &Tiles) -> Vec<Self> {
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

        let mut neighbors: Vec<Self> = vec![];

        for c in candidates {
            let (a, b) = *tiles.get(&map[c.x as usize][c.y as usize]).unwrap();
            if c.x + a.0 == self.x && c.y + a.1 == self.y
                || c.x + b.0 == self.x && c.y + b.1 == self.y
            {
                neighbors.push(c);
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 8);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 0);
    }
}
