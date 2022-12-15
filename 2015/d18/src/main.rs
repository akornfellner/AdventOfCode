fn main() {
    println!("part one: {}", solve("input.txt", 100, false));
    println!("part two: {}", solve("input.txt", 100, true));
}

use field::Field;

mod field;

fn solve(input: &str, steps: usize, two: bool) -> usize {
    let mut field: Field = Field::from(input);

    let rows = field.rows() - 1;
    let cols = field.cols() - 1;

    if two {
        field.set_light(0, 0, true);
        field.set_light(0, cols, true);
        field.set_light(rows, 0, true);
        field.set_light(rows, cols, true);
    }

    for _ in 0..steps {
        field = field.round(two);
    }

    field.count_lights()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", 4, false);
        assert_eq!(result, 4);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", 5, true);
        assert_eq!(result, 17);
    }
}
