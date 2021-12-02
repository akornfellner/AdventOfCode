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

    let lines: Vec<&str> = input.split('\n').collect();

    let mut accumulator = 0;

    let mut visited: Vec<i32> = vec![];

    let mut pos = 0;

    loop {
        if pos < 0 {
            break;
        }

        let p: usize = pos as usize;

        if visited.contains(&pos) || p >= lines.len() {
            break;
        }

        visited.push(pos);

        let cmd = Cmd::new_from_line(lines[p]);
        match cmd {
            Cmd::Nop(_value) => pos += 1,
            Cmd::Acc(value) => {
                accumulator += value;
                pos += 1;
            }
            Cmd::Jmp(value) => pos += value,
        }
    }

    println!("{}", accumulator);
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
