use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let lines: Vec<&str> = input.split('\n').collect();

    let mut h = 0;
    let mut d = 0;

    for line in lines {
        let cmd = Cmd::new_from_line(line);
        if &cmd.dir == "forward" {
            h += cmd.value;
        } else if &cmd.dir == "down" {
            d += cmd.value;
        } else if &cmd.dir == "up" {
            d -= cmd.value;
        }
    }

    println!("{}", h * d);
}

struct Cmd {
    dir: String,
    value: i32,
}

impl Cmd {
    fn new(dir: &str, value: i32) -> Self {
        Cmd {
            dir: String::from(dir),
            value,
        }
    }

    fn new_from_line(line: &str) -> Self {
        let values: Vec<&str> = line.split(' ').collect();
        let dir = values[0];
        let value: i32 = values[1].parse().unwrap();
        Cmd::new(dir, value)
    }
}
