use part_one::*;
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

    let mut bags: Vec<Bag> = vec![];

    for i in lines {
        bags.push(Bag::new_from_line(i));
    }

    let target = "shiny gold";
    let mut count = 0usize;

    for bag in &bags {
        if search_bag(&bag.name, target, &bags) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn search_bag(name: &str, target: &str, bags: &[Bag]) -> bool {
    if name == target {
        return false;
    }

    if Bag::get_bag(name, bags).is_empty() {
        return false;
    }

    for i in Bag::get_bag(name, bags) {
        if i == target {
            return true;
        }
    }

    let mut found = false;

    for i in Bag::get_bag(name, bags) {
        found = found || search_bag(&i, target, bags)
    }

    found
}
