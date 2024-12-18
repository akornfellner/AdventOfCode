use program::Program;
use std::env::args;

mod program;

fn main() {
    let filename = args().nth(1).unwrap_or("input.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (String, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut program = Program::compile(&input);

    let mut starts = vec![0];
    let mut len = 1;

    while len < program.operations.len() {
        let mut new_starts = Vec::new();
        for start in starts {
            for a in start..start + 8 {
                let mut p = program.clone();
                p.registers[0] = a;
                let output = p.run();
                if compare(&output, &p.operations) {
                    new_starts.push(8 * start + 8 * (a - start));
                }
            }
        }
        starts = new_starts;
        len += 1;
    }

    let mut min = usize::MAX;
    for start in starts {
        for a in start - 8..start + 16 {
            let mut p = program.clone();
            p.registers[0] = a;
            let output = p.run();
            if compare(&output, &p.operations) && a < min {
                min = a;
            }
        }
    }

    let output = program.run();
    let p1 = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    (p1, min)
}

fn compare(output: &[usize], operations: &[usize]) -> bool {
    let operations = operations.iter().rev().take(output.len());
    let output = output.iter().rev();

    operations.zip(output).all(|(x, y)| x == y)
}
