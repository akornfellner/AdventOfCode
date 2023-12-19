use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input_test.in");
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

    result
}

type Workflows = HashMap<String, Workflow>;

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
