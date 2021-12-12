use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn d1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader
        .read_to_string(&mut input)
        .expect("Reading Error");

    let numbers: Vec<i32> = input
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in &numbers {
        for j in &numbers {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    -1
}
#[cfg(test)]
mod tests {
    use crate::d1;

    #[test]
    fn it_works() {
        assert_eq!(d1("input_test.txt"), 514579);
    }
}
