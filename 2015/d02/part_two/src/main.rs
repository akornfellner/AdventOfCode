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
        .expect("Reading error");

    let lines: Vec<&str> = input.split('\n').collect();

    let mut result = 0usize;

    for line in lines {
        result += Present::new_from_line(line).get_size();
    }

    println!("{}", result);
}

#[derive(Debug, Clone)]
struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn new_from_line(line: &str) -> Self {
        let chars: Vec<&str> = line.split('x').collect();
        Present {
            l: chars[0].parse().unwrap(),
            w: chars[1].parse().unwrap(),
            h: chars[2].parse().unwrap(),
        }
    }

    fn get_size(&self) -> usize {
        let l = &self.l;
        let w = &self.w;
        let h = &self.h;
        let mut max = l;

        for i in [w, h] {
            if i > max {
                max = i;
            }
        }
        2 * (l + w + h) - 2 * max + l * w * h
    }
}
