mod bin;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::bin::*;

pub fn d3(file_name: &str) -> usize {
    let file = File::open(file_name).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let bins: Vec<Bin> = input.split('\n').map(|x| Bin::new_from_line(x)).collect();

    let (gamma, epsilon) = get_gamma_epsilon(&bins);

    let mut new = bins.clone();

    for i in 0..gamma.len() {
        let rest = new.clone();
        new = vec![];

        let (gamma, _epsilon) = get_gamma_epsilon(&rest);

        for bin in rest {
            if bin.get_bit(i) == gamma.get_bit(i) {
                new.push(bin.clone())
            }
        }

        if new.len() <= 1 {
            break;
        }
    }

    let oxygen = new[0].clone();

    let mut new = bins;

    for i in 0..epsilon.len() {
        let rest = new.clone();
        new = vec![];

        let (_gamma, epsilon) = get_gamma_epsilon(&rest);

        for bin in rest {
            if bin.get_bit(i) == epsilon.get_bit(i) {
                new.push(bin.clone())
            }
        }

        if new.len() <= 1 {
            break;
        }
    }

    let co2 = new[0].clone();

    oxygen.get_dec() * co2.get_dec()
}

fn get_gamma_epsilon(bins: &[Bin]) -> (Bin, Bin) {
    let mut gamma: Vec<usize> = vec![];
    let mut epsilon: Vec<usize> = vec![];

    for i in 0..bins[0].len() {
        let mut count_0 = 0usize;
        let mut count_1 = 0usize;

        for bin in bins {
            if bin.get_bit(i) == 0 {
                count_0 += 1;
            } else if bin.get_bit(i) == 1 {
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

    (gamma, epsilon)
}

#[cfg(test)]
mod tests {
    use crate::d3;

    #[test]
    fn it_works() {
        assert_eq!(d3("input_test.txt"), 230);
    }
}
