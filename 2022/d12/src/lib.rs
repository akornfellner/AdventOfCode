use pathfinding::prelude::dijkstra;
use std::fs;

pub fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let mut field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);

    for (i, row) in field.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = Pos(i as i32, j as i32);
            } else if *c == 'E' {
                end = Pos(i as i32, j as i32);
            }
        }
    }

    field[start.0 as usize][start.1 as usize] = 'a';
    field[end.0 as usize][end.1 as usize] = 'z';

    let mut result: i32 = 0;

    // in part one look for the shortest path between end and start with dijstra
    if !two {
        let path = dijkstra(&end, |p| p.neighbors(&field, false), |p| *p == start);
        if let Some((_, value)) = path {
            result = value as i32;
        }
    } else {
        // in part one look for the shortest path between end and position (-1,-1), this position is the position for all neighbors with value 'a'
        let path = dijkstra(&end, |p| p.neighbors(&field, true), |p| *p == Pos(-1, -1));
        if let Some((_, value)) = path {
            result = value as i32;
        }
    }

    result
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self, field: &[Vec<char>], two: bool) -> Vec<(Pos, usize)> {
        let mut neighbors: Vec<(Pos, usize)> = vec![];
        let mut directions: Vec<(i32, i32)> = vec![];

        if self.0 > 0 {
            directions.push((-1, 0));
        }
        if (self.0 as usize) < field.len() - 1 {
            directions.push((1, 0));
        }
        if self.1 > 0 {
            directions.push((0, -1));
        }

        if (self.1 as usize) < field[0].len() - 1 {
            directions.push((0, 1));
        }

        for direction in directions {
            if (field[(self.0 + direction.0) as usize][(self.1 + direction.1) as usize]) as i32
                - (field[(self.0) as usize][(self.1) as usize]) as i32
                >= -1
            {
                if two
                    && (field[(self.0 + direction.0) as usize][(self.1 + direction.1) as usize])
                        == 'a'
                {
                    neighbors.push((Pos(-1, -1), 1)) // if neighbor has value 'a' return Pos(-1,-1)
                } else {
                    neighbors.push((Pos(self.0 + direction.0, self.1 + direction.1), 1))
                }
            };
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 31);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 29);
    }
}
