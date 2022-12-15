use std::{cmp::Ordering, fs};

pub fn solve_one(input: &str, seconds: usize) -> usize {
    let reindeers = get_reindeers(input);

    let mut max = usize::MIN;

    for reindeer in reindeers {
        let value = reindeer.get_distance(seconds);

        if value > max {
            max = value;
        }
    }

    max
}

pub fn solve_two(input: &str, seconds: usize) -> usize {
    let mut reindeers = get_reindeers(input);

    for i in 1..=seconds {
        let mut max_index: Vec<usize> = vec![];
        let mut max_value = usize::MIN;

        for (j, reindeer) in reindeers.iter().enumerate() {
            let tmp = reindeer.get_distance(i);

            match tmp.cmp(&max_value) {
                Ordering::Equal => max_index.push(j),
                Ordering::Greater => {
                    max_index = vec![j];
                    max_value = tmp;
                }
                Ordering::Less => {}
            }
        }

        for value in max_index {
            reindeers[value].points += 1;
        }
    }

    let mut max = usize::MIN;

    for reindeer in reindeers {
        let value = reindeer.points;

        if value > max {
            max = value;
        }
    }

    max
}

fn get_reindeers(input: &str) -> Vec<Reindeer> {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut reindeers: Vec<Reindeer> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();

        let velocity: usize = parts[3].parse().unwrap();
        let sec: usize = parts[6].parse().unwrap();
        let rest: usize = parts[13].parse().unwrap();

        reindeers.push(Reindeer::new(velocity, sec, rest));
    }

    reindeers
}

#[derive(Debug)]
struct Reindeer {
    velocity: usize,
    sec: usize,
    rest: usize,
    points: usize,
}

impl Reindeer {
    fn new(velocity: usize, sec: usize, rest: usize) -> Self {
        Reindeer {
            velocity,
            sec,
            rest,
            points: 0,
        }
    }

    fn get_distance(&self, seconds: usize) -> usize {
        let intervalls = seconds as f64 / (self.sec + self.rest) as f64;
        let mut complete = intervalls as usize * self.sec * self.velocity;
        let rest = intervalls % 1.0;
        let rest = (rest * (self.sec + self.rest) as f64).round() as usize;

        if rest > self.sec {
            complete += self.sec * self.velocity;
        } else {
            complete += rest * self.velocity;
        }

        complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve_one("input_test.txt", 1000);
        assert_eq!(result, 1120);
    }

    #[test]
    fn two_works() {
        let result = solve_two("input_test.txt", 1000);
        assert_eq!(result, 689);
    }
}
