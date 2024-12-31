use pathfinding::prelude::bfs;
use std::{collections::HashMap, env::args};

fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, String) {
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

    let mut gates = parts[1].lines().map(Gate::from).collect::<Vec<_>>();

    let dest = get_decimal(&inputs, 'x') + get_decimal(&inputs, 'y');
    let p1 = run(&inputs, &gates);

    let mut ones = vec![];
    let mut twos = vec![];

    for gate in &gates {
        let (possible, rule) = check_gate(gate);
        if !possible {
            match rule {
                1 => ones.push(gate.output.clone()),
                2 => twos.push(gate.output.clone()),
                _ => {}
            }
        }
    }

    let mut swaps = vec![];

    for two in &twos {
        let result = bfs(
            two,
            |output| successors(output, &gates),
            |output| output.starts_with('z'),
        );
        let s = result.unwrap().iter().last().unwrap().to_string();
        let s = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;
        let s = format!("z{:02}", s);
        swap_outputs(&mut gates, two, &s);
        swaps.push(two.to_string());
        swaps.push(s);
    }

    let n = run(&inputs, &gates);
    let n = format!("{:b}", dest ^ n);

    let n = n.chars().rev().take_while(|&c| c == '0').count();

    let x = format!("x{n}");

    let mut last = vec![];

    for gate in &gates {
        if gate.inputs.0 == x || gate.inputs.1 == x {
            last.push(gate.output.clone());
        }
    }

    swap_outputs(&mut gates, &last[0], &last[1]);
    swaps.push(last[0].clone());
    swaps.push(last[1].clone());

    let n = run(&inputs, &gates);
    let t = dest - n;

    if t != 0 {
        panic!("The solution is wrong!");
    }

    swaps.sort();

    (p1, swaps.join(","))
}

fn swap_outputs(gates: &mut [Gate], first: &str, second: &str) {
    let a = gates.iter().position(|gate| gate.output == first).unwrap();
    let b = gates.iter().position(|gate| gate.output == second).unwrap();

    gates[a].output = second.to_string();
    gates[b].output = first.to_string();
}

fn check_gate(gate: &Gate) -> (bool, usize) {
    if gate.output != "z45" && gate.output.starts_with('z') {
        match gate.operation {
            Operation::Xor => {}
            _ => return (false, 1),
        }
    }

    if !(gate.output.starts_with('z')
        || gate.inputs.0.starts_with('x') && gate.inputs.1.starts_with('y')
        || gate.inputs.0.starts_with('y') && gate.inputs.1.starts_with('x'))
    {
        if let Operation::Xor = gate.operation {
            return (false, 2);
        }
    }

    (true, 0)
}

fn successors(output: &str, gates: &[Gate]) -> Vec<String> {
    let mut successors = vec![];
    for gate in gates {
        if gate.inputs.0 == output || gate.inputs.1 == output {
            successors.push(gate.output.clone());
        }
    }
    successors
}

fn run(inputs: &HashMap<String, usize>, gates: &[Gate]) -> usize {
    let mut inputs = inputs.clone();
    let mut gates = gates.to_vec();
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

    get_decimal(&inputs, 'z')
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

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
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
