use std::env::args;
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
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

    program.run();

    (program.get_output(), 0)
}

#[derive(Debug)]
struct Program {
    operations: Vec<usize>,
    registers: [usize; 3],
    pointer: usize,
    output: Vec<usize>,
}

impl Program {
    fn compile(input: &str) -> Self {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        let r = parts[0].lines().collect::<Vec<&str>>();
        let registers = [
            r[0].split_whitespace().last().unwrap().parse().unwrap(),
            r[1].split_whitespace().last().unwrap().parse().unwrap(),
            r[2].split_whitespace().last().unwrap().parse().unwrap(),
        ];

        let operations = parts[1].split_whitespace().collect::<Vec<&str>>();

        let operations = operations
            .iter()
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        Self {
            operations,
            registers,
            pointer: 0,
            output: vec![],
        }
    }

    fn combo(&self, index: usize) -> usize {
        match index {
            4..=6 => self.registers[index.wrapping_sub(4)],
            _ => index,
        }
    }

    fn operation(&mut self, opcode: usize, operand: usize) {
        let mut add = 2;
        match opcode {
            0 => {
                self.registers[0] = self.registers[0]
                    .checked_shr(self.combo(operand) as u32)
                    .unwrap_or(0);
            }
            1 => self.registers[1] ^= operand,
            2 => self.registers[1] = self.combo(operand) & 7,
            3 => {
                if self.registers[0] != 0 {
                    self.pointer = operand;
                    add = 0;
                }
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push(self.combo(operand) & 7),
            6 => {
                self.registers[1] = self.registers[0]
                    .checked_shr(self.combo(operand) as u32)
                    .unwrap_or(0);
            }

            _ => {
                self.registers[2] = self.registers[0]
                    .checked_shr(self.combo(operand) as u32)
                    .unwrap_or(0);
            }
        }
        self.pointer += add;
    }

    fn run(&mut self) {
        while self.pointer < self.operations.len() {
            let opcode = self.operations[self.pointer];
            let operand = self.operations[self.pointer + 1];
            self.operation(opcode, operand);
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
