use std::fs;

fn main() {
    let (result, crt) = solve("input.txt");
    println!("part one: {}\n", result);
    println!("part two:");
    println!("{crt}");
}

fn solve(input: &str) -> (i32, String) {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    let mut commands: Vec<Command> = lines.iter().map(|line| Command::from(line)).collect();

    let mut field = [[false; 40]; 6];
    let n = 240;

    let mut x = 1;
    let mut result = 0;
    let mut count = 0;
    let mut next = 0;
    let mut compare = 20;

    for k in 0..n {
        let i = k / 40;
        let j = k % 40;

        if count == 0 {
            x += next;
            let command = commands.remove(0);

            match command {
                Command::Noop => {
                    next = 0;
                    count = 1;
                }
                Command::Addx(value) => {
                    next = value;
                    count = 2;
                }
            }
        }
        count -= 1;

        if k + 1 == compare {
            result += x * (k + 1);
            compare += 40;
        }

        if [x - 1, x, x + 1].contains(&j) {
            field[i as usize][j as usize] = true;
        }
    }

    let mut crt = String::new();

    for line in field {
        for value in line {
            if value {
                crt += "#";
            } else {
                crt += "."
            }
        }
        crt += "\n";
    }

    (result, crt)
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn from(line: &[&str]) -> Self {
        match line[0] {
            "noop" => Self::Noop,
            _ => Self::Addx(line[1].parse::<i32>().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let (result, _) = solve("input_test.txt");
        assert_eq!(result, 13140);
    }
}
