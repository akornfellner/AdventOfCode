use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader.read_line(&mut input).expect("Reading error");

    let mut floor = 0;

    let mut position = 1;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        match floor {
            -1 => break,
            _ => position += 1,
        }
    }

    println!("{}", position);
}
