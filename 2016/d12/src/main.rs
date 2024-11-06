fn main() {
    println!("Part one: {}", solve("input.txt", false));
    println!("Part two: {}", solve("input.txt", true));
}

fn solve(file: &str, two: bool) -> i32 {
    let input = std::fs::read_to_string(file).unwrap();

    let mut program = Program::compile(&input);

    if two {
        program.register[2] = 1;
    }

    program.run();

    program.register[0]
}

#[derive(Debug)]
struct Program {
    cmds: Vec<Cmd>,
    register: [i32; 4],
    position: i32,
}

impl Program {
    fn compile(input: &str) -> Self {
        let cmds: Vec<Cmd> = input.lines().map(Cmd::new).collect();
        Program {
            cmds,
            register: [0; 4],
            position: 0,
        }
    }

    fn run(&mut self) {
        while self.position < self.cmds.len() as i32 {
            match self.cmds[self.position as usize] {
                Cmd::Cpy(Input::Val(val), reg) => self.register[reg] = val,
                Cmd::Cpy(Input::Reg(src), reg) => self.register[reg] = self.register[src],
                Cmd::Inc(reg) => self.register[reg] += 1,
                Cmd::Dec(reg) => self.register[reg] -= 1,
                Cmd::Jnz(Input::Val(val), offset) => {
                    if val != 0 {
                        self.position += offset;
                        continue;
                    }
                }
                Cmd::Jnz(Input::Reg(reg), offset) => {
                    if self.register[reg] != 0 {
                        self.position += offset;
                        continue;
                    }
                }
            }
            self.position += 1;
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Cpy(Input, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Input, i32),
}

impl Cmd {
    fn new(cmd: &str) -> Self {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts[0] {
            "cpy" => Cmd::Cpy(Input::new(parts[1]), get_index(parts[2])),
            "inc" => Cmd::Inc(get_index(parts[1])),
            "dec" => Cmd::Dec(get_index(parts[1])),
            "jnz" => Cmd::Jnz(Input::new(parts[1]), parts[2].parse::<i32>().unwrap()),
            _ => panic!("Invalid command"),
        }
    }
}

#[derive(Debug)]
enum Input {
    Reg(usize),
    Val(i32),
}

impl Input {
    fn new(input: &str) -> Self {
        match input.parse::<i32>() {
            Ok(val) => Input::Val(val),
            Err(_) => Input::Reg(get_index(input)),
        }
    }
}

fn get_index(reg: &str) -> usize {
    match reg {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register"),
    }
}
