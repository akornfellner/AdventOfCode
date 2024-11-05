use pathfinding::prelude::bfs;
use std::collections::HashMap;

fn main() {
    println!("Part one: {}", solve("input.txt", false));
    println!("Part two: {}", solve("input.txt", true));
}

fn solve(file: &str, two: bool) -> usize {
    let input = std::fs::read_to_string(file).unwrap();

    let mut start = State::from_input(&input);

    if two {
        start.1.push((1, 1));
        start.1.push((1, 1));
    }

    let goal = State(4, vec![(4, 4); start.size()]);

    let result = bfs(&start, |s| s.successors(), |s| *s == goal).unwrap();

    result.len() - 1
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State(i32, Vec<(i32, i32)>);

impl State {
    fn from_input(input: &str) -> Self {
        let mut positions: HashMap<String, (i32, i32)> = HashMap::new();

        for (floor, line) in input.lines().enumerate() {
            let floor = (floor + 1) as i32;
            if !line.contains(" a ") {
                continue;
            }
            let parts: Vec<&str> = line.split(" a ").collect();
            for part in parts.iter().skip(1) {
                let part = part.replace("-compatible", "").replace(['.', ','], "");
                let words = part.split_whitespace().collect::<Vec<&str>>();
                let ident = words[0].to_string();
                let entry = positions.entry(ident.clone()).or_insert((0, 0));
                match words[1] {
                    "generator" => {
                        entry.0 = floor;
                    }
                    "microchip" => {
                        entry.1 = floor;
                    }
                    _ => {
                        panic!("Unknown type: {}", words[1]);
                    }
                }
            }
        }

        Self(1, positions.values().copied().collect())
    }

    fn size(&self) -> usize {
        self.1.len()
    }

    fn is_valid(&self) -> bool {
        for (i, (gen, chip)) in self.1.iter().enumerate() {
            if *gen != *chip {
                for (j, (gen2, _)) in self.1.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    if *chip == *gen2 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn successors(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let floor = self.0;
        let mut floors = vec![self.0 - 1, self.0 + 1];
        if self.0 == 1 {
            floors.remove(0);
        }
        if self.0 == 4 {
            floors.remove(1);
        }

        for next_floor in floors {
            for (i, (gen, chip)) in self.1.iter().enumerate() {
                if *gen != floor && *chip != floor {
                    continue;
                }

                if *gen == floor && *chip == floor {
                    let mut next = self.clone();
                    next.0 = next_floor;
                    next.1[i].0 = next_floor;
                    next.1[i].1 = next_floor;
                    if next.is_valid() {
                        result.push(next);
                    }
                }

                for (j, (gen2, chip2)) in self.1.iter().enumerate().skip(i) {
                    if *gen2 != floor && *chip2 != floor {
                        continue;
                    }

                    if i == j {
                        let mut next = self.clone();
                        next.0 = next_floor;
                        if *gen == floor {
                            next.1[i].0 = next_floor;
                            if next.is_valid() {
                                result.push(next);
                            }
                        }

                        let mut next = self.clone();
                        next.0 = next_floor;
                        if *chip == floor {
                            next.1[i].1 = next_floor;
                            if next.is_valid() {
                                result.push(next);
                            }
                        }
                        continue;
                    }

                    if *gen == floor && *gen2 == floor {
                        let mut next = self.clone();
                        next.0 = next_floor;
                        next.1[i].0 = next_floor;
                        next.1[j].0 = next_floor;
                        if next.is_valid() {
                            result.push(next);
                        }
                    }

                    if *chip == floor && *chip2 == floor {
                        let mut next = self.clone();
                        next.0 = next_floor;
                        next.1[i].1 = next_floor;
                        next.1[j].1 = next_floor;
                        if next.is_valid() {
                            result.push(next);
                        }
                    }

                    if *gen == floor && *chip2 == floor {
                        let mut next = self.clone();
                        next.0 = next_floor;
                        next.1[i].0 = next_floor;
                        next.1[j].1 = next_floor;
                        if next.is_valid() {
                            result.push(next);
                        }
                    }

                    if *chip == floor && *gen2 == floor {
                        let mut next = self.clone();
                        next.0 = next_floor;
                        next.1[i].1 = next_floor;
                        next.1[j].0 = next_floor;
                        if next.is_valid() {
                            result.push(next);
                        }
                    }
                }
            }
        }

        result
    }
}
