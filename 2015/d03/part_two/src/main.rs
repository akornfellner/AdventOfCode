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

    let mut position_santa = Position::new(0, 0);
    let mut position_robo = Position::new(0, 0);

    let mut positions: Vec<Position> = vec![position_santa.clone()];

    let directions: Vec<char> = input.chars().collect();

    let mut houses = 1usize;

    let mut who = 0usize;

    for d in directions {
        let position: &mut Position;

        if who % 2 == 0 {
            position = &mut position_santa;
        } else {
            position = &mut position_robo;
        }

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

        who += 1;
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
