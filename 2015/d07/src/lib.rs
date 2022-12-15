use std::{collections::HashMap, fs};

pub fn solve_one(input: &str, wire: &str) -> u16 {
    let commands = get_commands(input);
    let mut results: HashMap<String, u16> = HashMap::new();

    exec_command(&commands, &mut results, wire)
}

pub fn solve_two(input: &str, wire: &str) -> u16 {
    let commands = get_commands(input);
    let mut results: HashMap<String, u16> = HashMap::new();

    let b = exec_command(&commands, &mut results, wire);

    results = HashMap::from([("b".to_string(), b)]);

    exec_command(&commands, &mut results, wire)
}

fn exec_command(
    commands: &HashMap<String, Command>,
    results: &mut HashMap<String, u16>,
    wire: &str,
) -> u16 {
    let result = wire.parse::<u16>();

    match result {
        Ok(value) => return value,
        Err(_) => {}
    }

    if results.contains_key(wire) {
        return *results.get(wire).unwrap();
    }

    let result = commands.get(wire);
    let command = result.unwrap().clone();

    let result = match command {
        Command::Assign(x) => exec_command(commands, results, &x),
        Command::Not(x) => !exec_command(commands, results, &x),
        Command::And(x, y) => {
            exec_command(commands, results, &x) & exec_command(commands, results, &y)
        }
        Command::Or(x, y) => {
            exec_command(commands, results, &x) | exec_command(commands, results, &y)
        }
        Command::Lshift(x, y) => exec_command(commands, results, &x) << y,
        Command::Rshift(x, y) => exec_command(commands, results, &x) >> y,
    };

    results.insert(wire.to_string(), result);

    result
}

fn get_commands(input: &str) -> HashMap<String, Command> {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut commands: HashMap<String, Command> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let dest: &str = parts[1];

        let parts: Vec<&str> = parts[0].split(' ').collect();

        match parts.len() {
            1 => {
                commands.insert(dest.to_string(), Command::Assign(parts[0].to_string()));
            }
            2 => {
                commands.insert(dest.to_string(), Command::Not(parts[1].to_string()));
            }
            _ => match parts[1] {
                "AND" => {
                    commands.insert(
                        dest.to_string(),
                        Command::And(parts[0].to_string(), parts[2].to_string()),
                    );
                }
                "OR" => {
                    commands.insert(
                        dest.to_string(),
                        Command::Or(parts[0].to_string(), parts[2].to_string()),
                    );
                }
                "LSHIFT" => {
                    commands.insert(
                        dest.to_string(),
                        Command::Lshift(parts[0].to_string(), parts[2].parse::<u16>().unwrap()),
                    );
                }
                _ => {
                    commands.insert(
                        dest.to_string(),
                        Command::Rshift(parts[0].to_string(), parts[2].parse::<u16>().unwrap()),
                    );
                }
            },
        }
    }

    commands
}

#[derive(Debug, Clone)]
enum Command {
    Assign(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve_one("input_test.txt", "i");
        assert_eq!(result, 65079);
    }
}
