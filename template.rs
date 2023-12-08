fn main() {
    let (p1, p2) = solve("input_test.in");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut result = (0, 0);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 0);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 0);
    }
}

