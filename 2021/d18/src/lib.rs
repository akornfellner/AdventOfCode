mod number;

use std::fs;

use crate::number::Number;

pub fn solve_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut number = Number::new(lines[0]);

    for line in &lines[1..] {
        number = number + Number::new(*line);
        number = Number::reduce(number);
    }

    number.magnitude()
}

pub fn solve_two(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut max = 0usize;

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i != j {
                let number = Number::new(lines[i]) + Number::new(lines[j]);
                let number = Number::reduce(number);
                let mag = number.magnitude();
                if mag > max {
                    max = mag;
                }
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::{solve_one, solve_two};

    #[test]
    fn one_works() {
        assert_eq!(solve_one("input_test.txt"), 4140);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve_two("input_test.txt"), 3993);
    }

    #[test]
    fn mag_1() {
        let number = Number::new("[[1,2],[[3,4],5]]");
        assert_eq!(number.magnitude(), 143);
    }

    #[test]
    fn mag_2() {
        let number = Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(number.magnitude(), 1384);
    }

    #[test]
    fn mag_3() {
        let number = Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(number.magnitude(), 445);
    }

    #[test]
    fn mag_4() {
        let number = Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(number.magnitude(), 791);
    }

    #[test]
    fn mag_5() {
        let number = Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(number.magnitude(), 1137);
    }

    #[test]
    fn mag_6() {
        let number = Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(number.magnitude(), 3488);
    }
}
