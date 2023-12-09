fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    for line in input.lines() {
        let (a, b) = History::from(line).predict();
        result.0 += a;
        result.1 += b;
    }

    result
}

#[derive(Debug)]
struct History {
    values: Vec<i64>,
}

impl History {
    fn from(line: &str) -> Self {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Self { values }
    }

    fn predict(&self) -> (i64, i64) {
        let mut current = self.values.clone();
        let mut lasts: Vec<i64> = vec![];
        let mut firsts: Vec<i64> = vec![];

        loop {
            let mut new: Vec<i64> = vec![];
            let mut finished = true;
            for i in 1..current.len() {
                let diff = current[i] - current[i - 1];
                if diff != 0 {
                    finished = false;
                }
                new.push(diff);
            }
            firsts.push(current[0]);
            lasts.push(current[current.len() - 1]);
            if finished {
                break;
            }
            current = new;
        }

        firsts.reverse();
        let mut first = 0;

        for f in firsts {
            first = f - first;
        }

        (lasts.iter().sum(), first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 114);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 2);
    }
}
