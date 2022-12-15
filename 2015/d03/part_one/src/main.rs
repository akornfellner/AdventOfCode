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

    let mut position = Position::new(0, 0);

    let mut positions: Vec<Position> = vec![position.clone()];

    let directions: Vec<char> = input.chars().collect();

    let mut houses = 1usize;

    for d in directions {
        match d {
            '^' => position.h += 1,
            '<' => position.w -= 1,
            '>' => position.w += 1,
            'v' => position.h -= 1,
            _ => {}
        }

        if !pos_used(&position, &positions) {
            houses += 1;
            positions.push(position.clone());
        }
    }

    println!("{}", houses);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    h: i32,
    w: i32,
}

impl Position {
    fn new(h: i32, w: i32) -> Self {
        Position { h: h, w: w }
    }
}

fn pos_used(position: &Position, positions: &[Position]) -> bool {
    let mut found = false;
    for p in positions {
        if p == position {
            found = true;
        }
    }
    found
}
