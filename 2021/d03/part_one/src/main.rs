mod bin;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::bin::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let bins: Vec<Bin> = input.split('\n').map(|x| Bin::new_from_line(x)).collect();

    let mut gamma: Vec<usize> = vec![];
    let mut epsilon: Vec<usize> = vec![];

    for i in 0..bins[0].length {
        let mut count_0 = 0usize;
        let mut count_1 = 0usize;

        for bin in &bins {
            if bin.bits[i] == 0 {
                count_0 += 1;
            } else if bin.bits[i] == 1 {
                count_1 += 1;
            }
        }

        if count_0 > count_1 {
            gamma.push(0);
            epsilon.push(1);
        } else {
            gamma.push(1);
            epsilon.push(0);
        }
    }

    let gamma = Bin::new(gamma);
    let epsilon = Bin::new(epsilon);

    println!("{}", gamma.get_dec() * epsilon.get_dec())
}
