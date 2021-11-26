use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Reading Errors");

    let seats: Vec<&str> = contents.split('\n').collect();

    let mut ids: Vec<i32> = vec![];

    for i in seats {
        let val = get_row(i) * 8 + get_col(i);
        ids.push(val);
    }

    ids.sort();

    let mut last_id = 0;

    for id in ids {
        if id - last_id > 1 {
            println!("{}", id - 1);
        }
        last_id = id;
    }
}

fn get_row(input: &str) -> i32 {
    let mut start = 0;
    let mut end = 127;
    let a: Vec<char> = input.chars().collect();
    for i in &a[..7] {
        let mean = (start + end) / 2;
        match i {
            'F' => end = mean,
            _ => start = mean + 1,
        }
    }
    start
}

fn get_col(input: &str) -> i32 {
    let mut start = 0;
    let mut end = 7;
    let a: Vec<char> = input.chars().collect();
    for i in &a[7..] {
        let mean = (start + end) / 2;
        match i {
            'L' => end = mean,
            _ => start = mean + 1,
        }
    }
    start
}
