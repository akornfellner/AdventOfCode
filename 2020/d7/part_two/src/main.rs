use part_two::*;
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

    let lines: Vec<&str> = input.split('n').collect();

    let mut bags: Vec<Bag> = vec![];

    for i in lines {
        bags.push(Bag::new_from_line(i));
    }

    let target = "shiny gold";

    println!("{}", count_bags(target, &bags) - 1);
}

fn count_bags(name: &str, bags: &[Bag]) -> usize {
    if Bag::get_bag(name, bags).is_empty() {
        return 1;
    }

    let mut result = 1usize;

    for (cur_name, number) in Bag::get_bag(name, bags) {
        result += count_bags(&cur_name, bags) * number;
    }

    result
}
