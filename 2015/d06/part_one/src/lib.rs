use std::fs;

pub fn solve(input: &str) -> usize {
    let commands = get_commands(input);

    let mut field = Field::new();

    for command in commands {
        field.exec_command(&command);
    }

    field.count()
}

fn get_commands(input: &str) -> Vec<Command> {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut commands: Vec<Command> = vec![];

    for line in lines {
        let start: (usize, usize);
        let end: (usize, usize);
        let light: Light;

        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "turn" {
            light = match parts[1] {
                "on" => Light::On,
                _ => Light::Off,
            };

            start = split_numbers(parts[2]);
            end = split_numbers(parts[4]);
        } else {
            light = Light::Toggle;
            start = split_numbers(parts[1]);
            end = split_numbers(parts[3]);
        }

        commands.push(Command::new(light, start, end));
    }

    commands
}

fn split_numbers(numbers: &str) -> (usize, usize) {
    let parts: Vec<&str> = numbers.split(',').collect();
    let x: usize = parts[0].parse().unwrap();
    let y: usize = parts[1].parse().unwrap();

    (x, y)
}

struct Field {
    field: Vec<Vec<bool>>,
}

impl Field {
    fn new() -> Self {
        Field {
            field: vec![vec![false; 1000]; 1000],
        }
    }

    fn exec_command(&mut self, command: &Command) {
        for i in command.start.0..=command.end.0 {
            for j in command.start.1..=command.end.1 {
                self.field[i][j] = match command.light {
                    Light::On => true,
                    Light::Off => false,
                    Light::Toggle => !self.field[i][j],
                }
            }
        }
    }

    fn count(&self) -> usize {
        let mut count = 0usize;

        for line in &self.field {
            for light in line {
                count += *light as usize;
            }
        }

        count
    }
}

#[derive(Debug)]
struct Command {
    light: Light,
    start: (usize, usize),
    end: (usize, usize),
}

impl Command {
    fn new(light: Light, start: (usize, usize), end: (usize, usize)) -> Self {
        Command { light, start, end }
    }
}

#[derive(Debug)]
enum Light {
    On,
    Off,
    Toggle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("input_test.txt");
        assert_eq!(result, 998996);
    }
}
