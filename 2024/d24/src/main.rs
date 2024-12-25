use std::{collections::HashMap, env::args};
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut inputs = HashMap::new();
    let parts = input.split("\n\n").collect::<Vec<_>>();

    for input in parts[0].lines() {
        let input = input.replace(':', "");
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let name = parts[0].to_string();
        let value = parts[1].parse::<usize>().unwrap();
        inputs.insert(name, value);
    }

    let dest = get_decimal(&inputs, 'x') + get_decimal(&inputs, 'y');

    let mut gates = parts[1].lines().map(Gate::from).collect::<Vec<_>>();

    for i in inputs.keys() {
        if i.starts_with('x') {
            let mut subs = vec![];
            get_subs(i, &gates, &mut subs);
            subs.sort();
            println!("{}: {:?}", i, subs);
        };
    }

    while !gates.is_empty() {
        let mut new_gates = Vec::new();
        for gate in gates.iter() {
            if gate.is_possible(&inputs) {
                let value = gate.calculate(&inputs);
                inputs.insert(gate.output.clone(), value);
            } else {
                new_gates.push(gate.clone());
            }
        }
        gates = new_gates;
    }

    (get_decimal(&inputs, 'z'), 0)
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

fn get_decimal(inputs: &HashMap<String, usize>, number: char) -> usize {
    let mut l = vec![];
    for (input, value) in inputs {
        if input.starts_with(number) {
            l.push((input.to_string(), *value));
        }
    }

    l.sort_by_key(|(input, _)| input.clone());

    let mut m = 1;
    let mut s = 0;

    for (_, value) in &l {
        s += m * value;
        m *= 2;
    }

    s
}

fn get_subs(input: &str, gates: &[Gate], subs: &mut Vec<String>) {
    if input.starts_with('z') {
        subs.push(input.to_string());
    } else {
        for gate in gates {
            if gate.inputs.0 == input || gate.inputs.1 == input {
                get_subs(&gate.output, gates, subs);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    operation: Operation,
    inputs: (String, String),
    output: String,
}

impl Gate {
    fn from(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let operation = match parts[1] {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Unknown operation"),
        };
        let inputs = (parts[0].to_string(), parts[2].to_string());
        let output = parts[4].to_string();

        Self {
            operation,
            inputs,
            output,
        }
    }

    fn is_possible(&self, inputs: &HashMap<String, usize>) -> bool {
        let (a, b) = &self.inputs;
        inputs.contains_key(a) && inputs.contains_key(b)
    }

    fn calculate(&self, inputs: &HashMap<String, usize>) -> usize {
        let (a, b) = &self.inputs;
        let a = *inputs.get(a).unwrap();
        let b = *inputs.get(b).unwrap();
        match self.operation {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        }
    }
}
