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

    let mut count = 0usize;

    for line in lines {
        if proof_nice(line) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn proof_nice(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    let mut double = false;
    let mut pair = false;

    let mut pairs = vec![(chars[0], chars[1])];

    for i in 2..chars.len() {
        let c = chars[i];
        if c == chars[i - 2] {
            double = true;
        }

        if i >= 3
            && (chars[i] == chars[i - 1] && chars[i] == chars[i - 2] && chars[i] == chars[i - 3])
        {
            pair = true
        }

        if pairs.contains(&(chars[i - 1], c))
            && !(chars[i] == chars[i - 1] && chars[i] == chars[i - 2])
            && !pair
        {
            pair = true;
        } else {
            pairs.push((chars[i - 1], c));
        }
    }

    double && pair
}
