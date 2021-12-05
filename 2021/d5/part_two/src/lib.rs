use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::line::Line;

mod line;

pub fn d5(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let strings: Vec<&str> = input.split('\n').collect();

    let mut count = 0;

    let mut points: Vec<(i32, i32)> = vec![];

    let mut points_checked: Vec<(i32, i32)> = vec![];

    for s in strings {
        let l = Line::new(s);
        let p = match l {
            Some(line) => line.points,
            None => vec![],
        };

        for point in p {
            if points.contains(&point) {
                if !(points_checked.contains(&point)) {
                    count += 1;
                    points_checked.push(point);
                }
            } else {
                {
                    points.push(point)
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::d5;

    #[test]
    fn it_works() {
        assert_eq!(d5("input_test.txt"), 12);
    }
}
