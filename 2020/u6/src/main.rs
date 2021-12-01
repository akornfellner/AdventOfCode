use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Reading errors");

    let answers: Vec<&str> = contents.split("\n\n").collect();

    let mut count = 0usize;

    for i in answers {
        let parts = my_split(i);
        count += count_equal_chars(&parts);
    }

    println!("{}", count);
}

fn my_split(input: &str) -> Vec<String> {
    let mut result = vec![];
    let arr: Vec<&str> = input.split('\n').collect();
    for i in arr {
        result.push(String::from(i));
    }
    result
}

fn count_equal_chars(input: &[String]) -> usize {
    let mut count = 0usize;
    for i in input[0].chars() {
        let mut e = true;
        for s in input.to_owned() {
            if !char_in_string(i, &s) {
                e = false;
            }
        }
        if e {
            count += 1;
        }
    }
    count
}

fn char_in_string(c: char, s: &str) -> bool {
    for i in s.chars() {
        if c == i {
            return true;
        }
    }
    false
}
