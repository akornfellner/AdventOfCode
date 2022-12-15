use std::fs;

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> String {
    let (cmds, mut stacks) = process_input(input);

    for cmd in cmds {
        for i in 0..cmd.amount {
            let cargo = stacks[cmd.src].remove(0);
            let index = if two { i } else { 0 };

            stacks[cmd.dest].insert(index, cargo);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(stack[0]);
    }

    result
}

fn process_input(input: &str) -> (Vec<Cmd>, Vec<Vec<char>>) {
    let input = fs::read_to_string(input).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let tmp: Vec<&str> = parts[0].lines().collect();
    let (tmp, n) = (
        &tmp[..tmp.len() - 1],
        tmp[tmp.len() - 1]
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );

    let cmds = parts[1];

    let mut stacks: Vec<Vec<char>> = vec![vec![]; n];

    for line in tmp {
        let chars: Vec<char> = line.chars().collect();

        for i in (0..line.len()).step_by(4) {
            if chars[i] == '[' {
                stacks[i / 4].push(chars[i + 1]);
            }
        }
    }

    let cmds: Vec<Cmd> = cmds.lines().map(Cmd::new).collect();

    (cmds, stacks)
}

#[derive(Debug)]
struct Cmd {
    amount: usize,
    src: usize,
    dest: usize,
}

impl Cmd {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        Self {
            amount: parts[1].parse::<usize>().unwrap(),
            src: parts[3].parse::<usize>().unwrap() - 1,
            dest: parts[5].parse::<usize>().unwrap() - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, String::from("CMZ"));
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, String::from("MCD"));
    }
}
