use std::collections::HashSet;

use pathfinding::prelude::bfs;

fn main() {
    println!("Part one: {}", solve(1352));
    println!("Part two: {}", solve2(1352));
}

fn solve(number: i32) -> usize {
    let start = Pos(1, 1);
    let goal = Pos(31, 39);

    let path = bfs(&start, |s| s.successors(number), |p| *p == goal).unwrap();

    path.len() - 1
}

fn solve2(number: i32) -> usize {
    let start = Pos(1, 1);

    let mut queue = vec![start];
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start);

    for _ in 0..50 {
        let mut next_queue = vec![];

        for pos in queue {
            for next_pos in pos.successors(number) {
                if visited.insert(next_pos) {
                    next_queue.push(next_pos);
                    visited.insert(next_pos);
                }
            }
        }

        queue = next_queue;
    }

    visited.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn is_free(x: i32, y: i32, number: i32) -> bool {
        let val = x * x + 3 * x + 2 * x * y + y + y * y + number;
        let ones = val.count_ones();
        ones % 2 == 0
    }

    fn successors(&self, number: i32) -> Vec<Pos> {
        let mut result = vec![];

        let (x, y) = (self.0, self.1);

        if x > 0 && Pos::is_free(x - 1, y, number) {
            result.push(Pos(x - 1, y));
        }

        if Pos::is_free(x + 1, y, number) {
            result.push(Pos(x + 1, y));
        }

        if y > 0 && Pos::is_free(x, y - 1, number) {
            result.push(Pos(x, y - 1));
        }

        if Pos::is_free(x, y + 1, number) {
            result.push(Pos(x, y + 1));
        }

        result
    }
}
