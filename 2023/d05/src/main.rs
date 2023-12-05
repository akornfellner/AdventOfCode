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

    let mut ranges: Vec<(usize, usize)> = vec![];

    for i in (0..seeds.len()).step_by(2) {
        ranges.push((seeds[i], seeds[i] + seeds[i + 1]));
    }

    for transformation in &transformations {
        ranges = transformation.transfrom_range(&ranges);
    }

    result.1 = ranges.iter().map(|r| r.0).min().unwrap();

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

    fn transfrom_range(&self, ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut ranges: Vec<(usize, usize)> = ranges.to_vec();
        let mut result: Vec<(usize, usize)> = vec![];

        for map in &self.mappings {
            let mut ranges_new: Vec<(usize, usize)> = vec![];

            for range in &ranges {
                let s = range.0;
                let e = range.1;
                let ms = map.start;
                let me = map.end;
                let d = map.diff;

                let before = (s, e.min(ms));
                let inter = (s.max(ms), e.min(me));
                let after = (s.max(me), e);

                if before.1 > before.0 {
                    ranges_new.push(before);
                }
                if inter.1 > inter.0 {
                    let new_start = (inter.0 as i64 + d) as usize;
                    let new_end = (inter.1 as i64 + d) as usize;
                    result.push((new_start, new_end));
                }
                if after.1 > after.0 {
                    ranges_new.push(after);
                }
            }
            ranges = ranges_new;
        }

        for range in ranges {
            result.push(range);
        }

        result
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
