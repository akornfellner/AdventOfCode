#[derive(Debug, Clone)]
pub struct Program {
    pub operations: Vec<usize>,
    pub registers: [usize; 3],
    pointer: usize,
}

impl Program {
    pub fn compile(input: &str) -> Self {
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
        }
    }

    fn combo(&self, index: usize) -> usize {
        match index {
            4..=6 => self.registers[index.wrapping_sub(4)],
            _ => index,
        }
    }

    fn operation(&mut self, opcode: usize, operand: usize, output: &mut Vec<usize>) {
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
            5 => output.push(self.combo(operand) & 7),
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

    pub fn run(&mut self) -> Vec<usize> {
        let mut output = vec![];
        while self.pointer < self.operations.len() {
            let opcode = self.operations[self.pointer];
            let operand = self.operations[self.pointer + 1];
            self.operation(opcode, operand, &mut output);
        }
        output
    }
}
