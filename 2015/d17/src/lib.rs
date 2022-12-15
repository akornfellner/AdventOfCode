use std::{collections::HashMap, fs};

pub fn solve(input: &str, amount: usize, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();

    let containers: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();

    let mut values: HashMap<usize, usize> = HashMap::new();

    get_combinations(&containers, 0, amount, 0, &mut values);

    if !two {
        return values.values().sum();
    }

    let min = values.keys().min().unwrap();

    values[min]
}

fn get_combinations(
    containers: &[usize],
    current: usize,
    amount: usize,
    count: usize,
    values: &mut HashMap<usize, usize>,
) {
    if current == amount {
        let entry = values.entry(count).or_insert(0);
        *entry += 1;
        return;
    } else if current > amount || containers.is_empty() {
        return;
    }

    let first = containers[0];
    let rest = &containers[1..];

    get_combinations(rest, current + first, amount, count + 1, values);
    get_combinations(rest, current, amount, count, values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", 25, false);
        assert_eq!(result, 4);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", 25, true);
        assert_eq!(result, 3);
    }
}
