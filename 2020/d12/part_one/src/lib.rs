use std::fs;

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut commands: Vec<Command> = vec![];

    for line in lines {
        let command = get_command(line);
        commands.push(command);
    }

    let mut position = Position::new();

    for command in commands {
        position.command(&command);
    }

    position.sum()
}

#[derive(Debug, Clone)]
struct Position {
    north: usize,
    east: usize,
    south: usize,
    west: usize,
    current: Direction,
}

impl Position {
    fn new() -> Self {
        Position {
            north: 0,
            east: 0,
            south: 0,
            west: 0,
            current: Direction::East,
        }
    }

    fn command(&mut self, command: &Command) {
        match command {
            Command::Forward(value) => {
                match self.current {
                    Direction::North => self.north += value,
                    Direction::East => self.east += value,
                    Direction::South => self.south += value,
                    Direction::West => self.west += value,
                };
            }
            Command::Move(direction, value) => match direction {
                Direction::North => self.north += value,
                Direction::East => self.east += value,
                Direction::South => self.south += value,
                Direction::West => self.west += value,
            },
            Command::Turn(turn, value) => {
                self.current = change_direction(&self.current, turn, *value)
            }
        }

        self.normalize();
    }

    fn normalize(&mut self) {
        if self.north > self.south {
            self.north -= self.south;
            self.south = 0;
        } else {
            self.south -= self.north;
            self.north = 0;
        }

        if self.east > self.west {
            self.east -= self.west;
            self.west = 0;
        } else {
            self.west -= self.east;
            self.east = 0;
        }
    }

    fn sum(&self) -> usize {
        self.north + self.east + self.south + self.west
    }
}

fn change_direction(current: &Direction, turn: &Turn, value: usize) -> Direction {
    let current = current.clone();
    let mut new = current as i32;

    let mut value = (value / 90) as i32;

    if let Turn::Left = turn {
        value *= -1;
    }

    new = (new + value + 4) % 4;

    Direction::new(new as usize)
}

fn get_command(input: &str) -> Command {
    let x: Vec<char> = input.chars().collect();
    let c = x[0];
    let v = &input[1..].parse::<usize>().unwrap();
    let v = *v;

    match c {
        'N' => Command::Move(Direction::North, v),
        'W' => Command::Move(Direction::West, v),
        'S' => Command::Move(Direction::South, v),
        'E' => Command::Move(Direction::East, v),
        'L' => Command::Turn(Turn::Left, v),
        'R' => Command::Turn(Turn::Right, v),
        _ => Command::Forward(v),
    }
}

#[derive(Debug, Clone)]
enum Command {
    Forward(usize),
    Move(Direction, usize),
    Turn(Turn, usize),
}

#[derive(Debug, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new(value: usize) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("input_test.txt");
        assert_eq!(result, 25);
    }
}
