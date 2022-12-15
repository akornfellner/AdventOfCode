use std::fs;

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut commands: Vec<Command> = vec![];

    for line in lines {
        let command = get_command(line);
        commands.push(command);
    }

    let mut waypoint = Position::new(1, 10, 0, 0);
    let mut boat = Position::new(0, 0, 0, 0);

    for command in commands {
        exec_command(&command, &mut waypoint, &mut boat);
    }

    boat.sum()
}

fn exec_command(command: &Command, waypoint: &mut Position, boat: &mut Position) {
    match command {
        Command::Forward(value) => {
            boat.north += value * waypoint.north;
            boat.east += value * waypoint.east;
            boat.south += value * waypoint.south;
            boat.west += value * waypoint.west;
        }
        Command::Move(direction, value) => match direction {
            Direction::North => waypoint.north += value,
            Direction::East => waypoint.east += value,
            Direction::South => waypoint.south += value,
            Direction::West => waypoint.west += value,
        },
        Command::Turn(turn, value) => {
            let north = waypoint.north;
            let east = waypoint.east;
            let south = waypoint.south;
            let west = waypoint.west;

            match turn {
                Turn::Right => match value {
                    90 => {
                        waypoint.north = west;
                        waypoint.east = north;
                        waypoint.south = east;
                        waypoint.west = south;
                    }
                    180 => {
                        waypoint.north = south;
                        waypoint.east = west;
                        waypoint.south = north;
                        waypoint.west = east;
                    }
                    _ => {
                        waypoint.north = east;
                        waypoint.east = south;
                        waypoint.south = west;
                        waypoint.west = north;
                    }
                },
                Turn::Left => match value {
                    90 => {
                        waypoint.north = east;
                        waypoint.east = south;
                        waypoint.south = west;
                        waypoint.west = north;
                    }
                    180 => {
                        waypoint.north = south;
                        waypoint.east = west;
                        waypoint.south = north;
                        waypoint.west = east;
                    }
                    _ => {
                        waypoint.north = west;
                        waypoint.east = north;
                        waypoint.south = east;
                        waypoint.west = south;
                    }
                },
            }
        }
    };

    waypoint.normalize();
    boat.normalize();
}

#[derive(Debug, Clone)]
struct Position {
    north: usize,
    east: usize,
    south: usize,
    west: usize,
}

impl Position {
    fn new(north: usize, east: usize, south: usize, west: usize) -> Self {
        Position {
            north,
            east,
            south,
            west,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("input_test.txt");
        assert_eq!(result, 286);
    }
}
