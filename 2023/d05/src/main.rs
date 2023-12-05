use std::fs;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = fs::read_to_string(filename).unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut result = (usize::MAX, usize::MAX);

    let seeds: Vec<usize> = parts[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut transformations: Vec<Transformation> = vec![];

    for part in parts.iter().skip(1) {
        transformations.push(Transformation::new(part));
    }

    for seed in &seeds {
        let mut x = *seed;
        for transformation in &transformations {
            x = transformation.transform(x);
        }
        if x < result.0 {
            result.0 = x;
        }
    }

    for i in (0..seeds.len()).step_by(2) {
        for s in seeds[i]..seeds[i] + seeds[i + 1] {
            let mut x = s;
            for transformation in &transformations {
                x = transformation.transform(x);
            }
            if x < result.1 {
                result.1 = x;
            }
        }
    }

    result
}

#[derive(Debug)]
struct Mapping {
    start: usize,
    end: usize,
    diff: i64,
}

impl Mapping {
    fn new(line: &str) -> Self {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Self {
            start: parts[1],
            end: parts[1] + parts[2],
            diff: parts[0] as i64 - parts[1] as i64,
        }
    }
}

#[derive(Debug)]
struct Transformation {
    mappings: Vec<Mapping>,
}

impl Transformation {
    fn new(input: &str) -> Self {
        let mut mappings = vec![];
        for line in input.lines().skip(1) {
            mappings.push(Mapping::new(line));
        }
        Self { mappings }
    }

    fn transform(&self, x: usize) -> usize {
        let mut x = x;
        for map in &self.mappings {
            if x >= map.start && x < map.end {
                x = (x as i64 + map.diff) as usize;
                break;
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 35);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 46);
    }
}
