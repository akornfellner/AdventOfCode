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

    let mut boards: Vec<(Board, bool)> = vec![];

    for b in &fields[1..] {
        boards.push((Board::new_from_string(b), false));
    }

    let mut count = 0;
    let size = boards.len();

    for number in numbers {
        for board in &mut boards {
            board.0.new_number(number);
            if board.0.won() && !board.1 {
                board.1 = true;
                count += 1;
            }
            if count == size {
                return board.0.sum_unmarked() * number;
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
        assert_eq!(d4("input_test.txt"), 1924);
    }
}
