use std::{
    fs::File,
    io::{BufReader, Read},
    vec,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let mut commands: Vec<Cmd> = input.split('\n').map(|x| Cmd::new_from_line(x)).collect();

    let mut accumulator: i32;

    let size = commands.len();

    'outer: for i in 0..size {
        commands[i] = match commands[i] {
            Cmd::Nop(value) => Cmd::Jmp(value),
            Cmd::Acc(value) => Cmd::Acc(value),
            Cmd::Jmp(value) => Cmd::Nop(value),
        };

        accumulator = 0;
        let mut visited: Vec<i32> = vec![];
        let mut pos = 0;

        'inner: loop {
            let p: usize = pos as usize;

            if p == commands.len() - 1 {
                println!("{}", accumulator);
            }

            if pos < 0 {
                break 'inner;
            }

            if visited.contains(&pos) {
                break 'inner;
            }

            if p >= commands.len() {
                break 'outer;
            }

            visited.push(pos);

            match commands[p] {
                Cmd::Nop(_value) => pos += 1,
                Cmd::Acc(value) => {
                    accumulator += value;
                    pos += 1;
                }
                Cmd::Jmp(value) => pos += value,
            }
        }

        commands[i] = match commands[i] {
            Cmd::Nop(value) => Cmd::Jmp(value),
            Cmd::Acc(value) => Cmd::Acc(value),
            Cmd::Jmp(value) => Cmd::Nop(value),
        };
    }
}

#[derive(Debug)]
enum Cmd {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Cmd {
    fn new_from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        let cmd = parts[0];
        let number = parts[1];
        let a: Vec<char> = number.chars().collect();
        let sign = a[0];
        let number = &number[1..];

        let mut value: i32 = number.parse().unwrap();

        if sign == '-' {
            value *= -1;
        }

        match cmd {
            "nop" => Cmd::Nop(value),
            "acc" => Cmd::Acc(value),
            "jmp" => Cmd::Jmp(value),
            _ => Cmd::Nop(0),
        }
    }
}
