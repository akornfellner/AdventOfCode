use std::fs;

pub fn solve_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let a = 1;
    let b = 2;
    let c = 3;

    5
}

#[cfg(test)]
mod tests {
    use crate::solve_one;

    #[test]
    fn one_works() {
        assert_eq!(solve_one("input_test.txt"), 4);
    }
}
