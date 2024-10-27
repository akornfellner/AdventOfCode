fn main() {
    println!("Part one: {}", solve("input.txt"));
}

fn solve(file: &str) -> usize {
    let input = std::fs::read_to_string(file).unwrap();
    let mut screen = [[false; 50]; 6];

    let instructions: Vec<Instruction> = input.lines().map(Instruction::from_line).collect();

    for instruction in instructions {
        match instruction {
            Instruction::Rect(width, height) => {
                for i in 0..height {
                    for j in 0..width {
                        screen[i][j] = true;
                    }
                }
            }
            Instruction::RotateRow(index, amount) => {
                let mut new_screen = screen.clone();
                for i in 0..screen[0].len() {
                    let k = (i + amount) % 50;
                    new_screen[index][k] = screen[index][i];
                }
                screen = new_screen;
            }
            Instruction::RotateColumn(index, amount) => {
                let mut new_screen = screen.clone();
                for i in 0..screen.len() {
                    let k = (i + amount) % 6;
                    new_screen[k][index] = screen[i][index];
                }
                screen = new_screen;
            }
        }
    }

    let mut count = 0;

    for row in screen {
        for value in row {
            if value {
                count += 1;
            }
        }
    }

    print_screen(&screen);

    count
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "rect" => {
                let dims: Vec<usize> = parts[1].split("x").map(|x| x.parse().unwrap()).collect();
                Instruction::Rect(dims[0], dims[1])
            }
            "rotate" => {
                let index: usize = parts[2].split("=").nth(1).unwrap().parse().unwrap();
                let amount: usize = parts[4].parse().unwrap();
                match parts[1] {
                    "row" => Instruction::RotateRow(index, amount),
                    "column" => Instruction::RotateColumn(index, amount),
                    _ => panic!("Invalid instruction"),
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }
}

fn print_screen(screen: &[[bool; 50]; 6]) {
    for row in screen {
        for pixel in row {
            print!("{}", if *pixel { "#" } else { "." });
        }
        println!();
    }
}
