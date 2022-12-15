use std::fs;

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut elves: Vec<usize> = vec![];

    for part in parts {
        let e: usize = part.lines().map(|x| x.parse::<usize>().unwrap()).sum();
        elves.push(e);
    }

    if !two {
        return elves.into_iter().max().unwrap();
    }

    elves.sort();
    elves.reverse();

    elves[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 24000);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 45000);
    }
}
