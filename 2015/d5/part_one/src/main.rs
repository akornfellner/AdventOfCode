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
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut count_vocals = 0usize;
    let mut double_letter = false;
    let mut contain_bad_strings = false;

    let mut last_c = '0';

    for c in input.chars() {
        for v in vowels {
            if c == v {
                count_vocals += 1;
            }
        }

        if c == last_c {
            double_letter = true;
        }

        if (c == 'b' && last_c == 'a')
            || (c == 'd' && last_c == 'c')
            || (c == 'q' && last_c == 'p')
            || (c == 'y' && last_c == 'x')
        {
            contain_bad_strings = true;
        }

        last_c = c;
    }

    count_vocals >= 3 && double_letter && !contain_bad_strings
}
