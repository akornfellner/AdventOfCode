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
    let mut numbers: Vec<i32> = vec![];

    for i in lines {
        let number: i32 = i.parse().unwrap();
        numbers.push(number);
    }

    let mut count = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
