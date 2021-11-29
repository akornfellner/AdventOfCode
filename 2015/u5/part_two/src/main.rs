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

    let lines: Vec<&str> = input.split("\n").collect();

    let mut count = 0usize;

    for line in lines {
        if proof_nice(line) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn proof_nice(input: &str) -> bool {
    let mut repeat_char = false;
    let mut double_pair = false;

    let mut pairs: Vec<(char, char)> = vec![];
    let mut last_pair = ('1', '1');

    let mut lc = '0';
    let mut llc = '0';

    for c in input.chars() {
        if c == llc {
            repeat_char = true;
        }

        let pair = (lc, c);

        if pairs.contains(&pair) {
            double_pair = true;
        } else {
            pairs.push(pair);
        }

        last_pair = pair;
        llc = lc;
        lc = c;
    }

    repeat_char && double_pair
}
