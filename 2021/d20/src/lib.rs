mod picture;

use std::fs;

use crate::picture::Picture;

pub fn solve(filename: &str, rounds: usize) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let input: Vec<&str> = input.split("\n\n").collect();
    let algo_string = input[0];
    let picture = input[1];

    let mut algo: Vec<bool> = vec![];

    for c in algo_string.chars() {
        algo.push(matches!(c, '#'));
    }

    let mut picture = Picture::new(picture);

    picture.calculate(&algo, rounds);

    picture.count()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn one_works() {
        assert_eq!(solve("input_test.txt", 2), 35);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve("input_test.txt", 50), 3351);
    }
}
