use std::vec;

use pathfinding::prelude::dijkstra;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let field: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = Node {
        x: 0,
        y: 0,
        direction: Direction::Down,
        steps: 0,
    };

    let a = dijkstra(
        &start,
        |n| n.neighbors(&field, false),
        |n| n.x == field.len() - 1 && n.y == field[0].len() - 1,
    )
    .unwrap();

    result.0 = a.1;

    let a = dijkstra(
        &start,
        |n| n.neighbors(&field, true),
        |n| n.x == field.len() - 1 && n.y == field[0].len() - 1,
    )
    .unwrap();

    result.1 = a.1;

    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    steps: usize,
}

impl Node {
    fn neighbors(&self, field: &[Vec<usize>], two: bool) -> Vec<(Node, usize)> {
        let min = if !two || self.steps == 0 { 0 } else { 4 };
        let max = if two { 10 } else { 3 };

        let xmax = field.len() - 1;
        let ymax = field[0].len() - 1;
        let mut result: Vec<(Node, usize)> = vec![];

        match self.direction {
            Direction::Up => {
                if self.steps < max && self.x > 0 {
                    result.push((
                        Node {
                            x: self.x - 1,
                            y: self.y,
                            direction: Direction::Up,
                            steps: self.steps + 1,
                        },
                        field[self.x - 1][self.y],
                    ));
                }
                if self.y > 0 && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y - 1,
                            direction: Direction::Left,
                            steps: 1,
                        },
                        field[self.x][self.y - 1],
                    ));
                }
                if self.y < ymax && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y + 1,
                            direction: Direction::Right,
                            steps: 1,
                        },
                        field[self.x][self.y + 1],
                    ));
                }
            }
            Direction::Down => {
                if self.steps < max && self.x < xmax {
                    result.push((
                        Node {
                            x: self.x + 1,
                            y: self.y,
                            direction: Direction::Down,
                            steps: self.steps + 1,
                        },
                        field[self.x + 1][self.y],
                    ));
                }
                if self.y > 0 && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y - 1,
                            direction: Direction::Left,
                            steps: 1,
                        },
                        field[self.x][self.y - 1],
                    ));
                }
                if self.y < ymax && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y + 1,
                            direction: Direction::Right,
                            steps: 1,
                        },
                        field[self.x][self.y + 1],
                    ));
                }
            }
            Direction::Left => {
                if self.steps < max && self.y > 0 {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y - 1,
                            direction: Direction::Left,
                            steps: self.steps + 1,
                        },
                        field[self.x][self.y - 1],
                    ));
                }
                if self.x > 0 && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x - 1,
                            y: self.y,
                            direction: Direction::Up,
                            steps: 1,
                        },
                        field[self.x - 1][self.y],
                    ));
                }
                if self.x < xmax && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x + 1,
                            y: self.y,
                            direction: Direction::Down,
                            steps: 1,
                        },
                        field[self.x + 1][self.y],
                    ));
                }
            }
            Direction::Right => {
                if self.steps < max && self.y < ymax {
                    result.push((
                        Node {
                            x: self.x,
                            y: self.y + 1,
                            direction: Direction::Right,
                            steps: self.steps + 1,
                        },
                        field[self.x][self.y + 1],
                    ));
                }
                if self.x > 0 && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x - 1,
                            y: self.y,
                            direction: Direction::Up,
                            steps: 1,
                        },
                        field[self.x - 1][self.y],
                    ));
                }
                if self.x < xmax && self.steps >= min {
                    result.push((
                        Node {
                            x: self.x + 1,
                            y: self.y,
                            direction: Direction::Down,
                            steps: 1,
                        },
                        field[self.x + 1][self.y],
                    ));
                }
            }
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
