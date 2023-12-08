use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0].chars().collect::<Vec<char>>();

    let mut result = (0, 1);

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in parts[1].lines() {
        let (node, left, right) = parse_line(line);
        nodes.insert(node, (left, right));
    }

    result.0 = transform("AAA", &instructions, &nodes, false);

    let mut current: Vec<String> = vec![];

    for key in nodes.keys() {
        if key.ends_with('A') {
            current.push(key.to_string());
        }
    }

    let mut counts: Vec<usize> = vec![];

    for c in current {
        counts.push(transform(&c, &instructions, &nodes, true));
    }

    for c in counts {
        result.1 = lcm(result.1, c);
    }

    result
}

fn parse_line(line: &str) -> (String, String, String) {
    let line = line.replace(['=', '(', ')', ','], "");
    let parts = line.split_whitespace().collect::<Vec<&str>>();

    (
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    )
}

fn transform(
    start: &str,
    instructions: &[char],
    nodes: &HashMap<String, (String, String)>,
    two: bool,
) -> usize {
    let mut count = 0;
    let mut current = start.to_string();
    loop {
        let instruction = instructions[count % instructions.len()];
        let (left, right) = nodes.get(&current).unwrap();
        match instruction {
            'L' => current = left.to_string(),
            'R' => current = right.to_string(),
            _ => (),
        }
        count += 1;
        if current == "ZZZ" && !two {
            return count;
        }
        if current.ends_with('Z') && two {
            return count;
        }
    }
}
