use std::{collections::HashMap, fs};

pub fn solve(input: &str, rounds: usize, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();

    // we can keep track of the number by keeping track of the number modulo the lcm of all test values
    let mut lcm = 1usize;
    for monkey in &monkeys {
        lcm *= monkey.test;
    }

    let mut inspects: HashMap<usize, usize> = HashMap::new();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let inspect = inspects.entry(i).or_insert(0);
                *inspect += 1;

                let item = monkeys[i].items.remove(0);
                let new = (match monkeys[i].operation {
                    Operation::Plus(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::Square => item * item,
                }) / if two { 1 } else { 3 }
                    % lcm;

                let dest = if new % monkeys[i].test == 0 {
                    monkeys[i].dest.0
                } else {
                    monkeys[i].dest.1
                };

                monkeys[dest].items.push(new);
            }
        }
    }

    let mut values: Vec<usize> = inspects.into_values().collect();
    values.sort();
    values.reverse();

    let mut result = 1;

    for value in &values[..2] {
        result *= value;
    }

    result
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    dest: (usize, usize),
}

impl Monkey {
    fn from(input: &str) -> Self {
        let mut lines: Vec<String> = input.lines().map(|line| line.trim().to_owned()).collect();

        lines[1] = lines[1].replace(',', "");
        let items: Vec<usize> = lines[1]
            .split(' ')
            .skip(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let op: Vec<&str> = lines[2].split(' ').skip(4).collect();
        let operation = match op[0] {
            "+" => Operation::Plus(op[1].parse::<usize>().unwrap()),
            _ => match op[1] {
                "old" => Operation::Square,
                _ => Operation::Multiply(op[1].parse::<usize>().unwrap()),
            },
        };

        let test = lines[3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let t = lines[4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let f = lines[5]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let dest = (t, f);

        Self {
            items,
            operation,
            test,
            dest,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Plus(usize),
    Multiply(usize),
    Square,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", 20, false);
        assert_eq!(result, 10605);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", 10000, true);
        assert_eq!(result, 2713310158);
    }
}
