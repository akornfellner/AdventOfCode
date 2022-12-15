use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::board::Board;

mod board;

pub fn d4(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let fields: Vec<&str> = input.split("\n\n").collect();
    let numbers: Vec<i32> = fields[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    for b in &fields[1..] {
        boards.push(Board::new_from_string(b))
    }

    for number in numbers {
        for board in &mut boards {
            board.new_number(number);
            if board.won() {
                return number * board.sum_unmarked();
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::d4;

    #[test]
    fn it_works() {
        assert_eq!(d4("input_test.txt"), 4512);
    }
}
