use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut workflows: Workflows = HashMap::new();
    let mut ratings: Vec<Rating> = vec![];

    for line in parts[1].lines() {
        ratings.push(Rating::from(line));
    }

    for line in parts[0].lines() {
        let line = line.replace('{', " {");
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let workflow = Workflow::from(parts[1]);
        workflows.insert(name, workflow);
    }

    for rating in ratings {
        if run_workflow("in", &rating, &workflows) {
            result.0 += rating.x + rating.m + rating.a + rating.s;
        }
    }

    let mut start = RatingRange::from(1, 4000);

    result.1 = run_workflow_range("in", &mut start, &workflows);

    result
}

type Workflows = HashMap<String, Workflow>;
type Ranges = (Option<(usize, usize)>, Option<(usize, usize)>);

fn run_workflow_range(key: &str, rr: &mut RatingRange, workflows: &Workflows) -> usize {
    let workflow = workflows.get(key).unwrap();
    let mut result = 0;

    for rule in &workflow.rules {
        match rule {
            Rule::Comparison(c, op, value, next) => {
                let index: usize = match c {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    _ => 3,
                };

                let (transform, rest) = transfrom_range(rr.ratings[index], *op, *value);

                if let Some(t) = transform {
                    if let Some(r) = rest {
                        rr.ratings[index] = r;
                        let mut new_rr = rr.clone();
                        new_rr.ratings[index] = t;
                        if next == "A" {
                            result += new_rr.count();
                        } else if next != "R" {
                            result += run_workflow_range(next, &mut new_rr, workflows);
                        }
                    } else if next == "A" {
                        return rr.count();
                    } else if next == "R" {
                        return 0;
                    } else {
                        return run_workflow_range(next, rr, workflows);
                    }
                } else {
                    rr.ratings[index] = rest.unwrap();
                }
            }
            Rule::Empty(next) => {
                if next == "A" {
                    return result + rr.count();
                } else if next == "R" {
                    return result;
                }
                return result + run_workflow_range(next, rr, workflows);
            }
        }
    }

    result
}

fn transfrom_range(range: (usize, usize), op: char, value: usize) -> Ranges {
    match op {
        '<' => {
            if range.1 < value {
                (Some(range), None)
            } else if range.0 >= value {
                (None, Some(range))
            } else {
                (Some((range.0, value - 1)), Some((value, range.1)))
            }
        }
        '>' => {
            if range.0 > value {
                (Some(range), None)
            } else if range.1 <= value {
                (None, Some(range))
            } else {
                (Some((value + 1, range.1)), Some((range.0, value)))
            }
        }
        _ => (None, None),
    }
}

#[derive(Debug, Clone)]
struct RatingRange {
    ratings: Vec<(usize, usize)>,
}

impl RatingRange {
    fn count(&self) -> usize {
        let mut count = 1;
        for rating in &self.ratings {
            count *= rating.1 - rating.0 + 1;
        }
        count
    }
}

impl RatingRange {
    fn from(start: usize, end: usize) -> Self {
        Self {
            ratings: vec![(start, end); 4],
        }
    }
}

fn run_workflow(key: &str, rating: &Rating, workflows: &Workflows) -> bool {
    let workflow = workflows.get(key).unwrap();

    for rule in &workflow.rules {
        match rule {
            Rule::Comparison(r, op, value, next) => {
                let r = match r {
                    'x' => rating.x,
                    'm' => rating.m,
                    'a' => rating.a,
                    's' => rating.s,
                    _ => 0,
                };
                match op {
                    '<' => {
                        if r < *value {
                            if next == "A" {
                                return true;
                            } else if next == "R" {
                                return false;
                            }
                            return run_workflow(next, rating, workflows);
                        } else {
                            continue;
                        }
                    }
                    '>' => {
                        if r > *value {
                            if next == "A" {
                                return true;
                            } else if next == "R" {
                                return false;
                            }
                            return run_workflow(next, rating, workflows);
                        } else {
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            Rule::Empty(next) => match next.as_str() {
                "A" => return true,
                "R" => return false,
                _ => return run_workflow(next, rating, workflows),
            },
        }
    }

    false
}

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn from(input: &str) -> Self {
        let input = input.replace(['{', '}'], "");
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for part in input.split(',') {
            let parts = part.split('=').collect::<Vec<&str>>();
            match parts[0] {
                "x" => x = parts[1].parse().unwrap(),
                "m" => m = parts[1].parse().unwrap(),
                "a" => a = parts[1].parse().unwrap(),
                "s" => s = parts[1].parse().unwrap(),
                _ => (),
            }
        }
        Self { x, m, a, s }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(input: &str) -> Self {
        let mut rules = vec![];
        let input = input.replace('}', "");
        let parts: Vec<&str> = input.split('{').collect();
        for rule in parts[1].split(',') {
            rules.push(Rule::from(rule));
        }

        Self { rules }
    }
}

#[derive(Debug)]
enum Rule {
    Comparison(char, char, usize, String),
    Empty(String),
}

impl Rule {
    fn from(input: &str) -> Self {
        if input.contains('<') {
            let input = input.replace(':', "<");
            let parts: Vec<&str> = input.split('<').collect();
            let rating = parts[0].chars().next().unwrap();
            let value: usize = parts[1].parse().unwrap();
            let next = parts[2].to_string();
            Self::Comparison(rating, '<', value, next)
        } else if input.contains('>') {
            let input = input.replace(':', ">");
            let parts: Vec<&str> = input.split('>').collect();
            let rating = parts[0].chars().next().unwrap();
            let value: usize = parts[1].parse().unwrap();
            let next = parts[2].to_string();
            Self::Comparison(rating, '>', value, next)
        } else {
            Self::Empty(input.to_string())
        }
    }
}
