mod command;

use std::{collections::HashSet, fs};

use command::Command;

pub fn solve_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut cmds: Vec<Command> = vec![];

    for line in lines {
        cmds.push(Command::new(line));
    }

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for cmd in cmds {
        let startx = match cmd.x.0 >= -50 {
            true => cmd.x.0,
            false => -50,
        };
        let endx = match cmd.x.0 <= 50 {
            true => cmd.x.1,
            false => 50,
        };

        let starty = match cmd.y.0 >= -50 {
            true => cmd.y.0,
            false => -50,
        };
        let endy = match cmd.y.0 <= 50 {
            true => cmd.y.1,
            false => 50,
        };

        let startz = match cmd.z.0 >= -50 {
            true => cmd.z.0,
            false => -50,
        };
        let endz = match cmd.z.0 <= 50 {
            true => cmd.z.1,
            false => 50,
        };

        for x in startx..=endx {
            for y in starty..=endy {
                for z in startz..=endz {
                    if cmd.on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len()
}

pub fn solve_two(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut cmds: Vec<Command> = vec![];

    for line in lines {
        cmds.push(Command::new(line));
    }

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    let rounds = cmds.len();
    let mut round = 1;

    for cmd in cmds {
        println!("Round {} of {}", round, rounds);
        for x in cmd.x.0..=cmd.x.1 {
            for y in cmd.y.0..=cmd.y.1 {
                for z in cmd.z.0..=cmd.z.1 {
                    if cmd.on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
        round += 1;
    }

    cubes.len()
}

#[cfg(test)]
mod tests {
    use crate::{solve_one, solve_two};

    #[test]
    fn one_works() {
        assert_eq!(solve_one("test_one.txt"), 590784);
    }

    #[test]
    fn two_works() {
        assert_eq!(solve_two("test_two.txt"), 2758514936282235);
    }
}
