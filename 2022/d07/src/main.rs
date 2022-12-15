use std::{collections::HashMap, fs};

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let commands: Vec<&str> = input.split('$').map(|x| x.trim()).collect();
    let commands = &commands[2..];

    let mut path = Path::new();
    let mut tree: HashMap<String, usize> = HashMap::new();

    for command in commands {
        if &command[..2] == "cd" {
            let parts: Vec<&str> = command.split(' ').collect();
            let dest = parts[1];
            if dest == ".." {
                path.pop();
            } else {
                path.push(dest);
            }
        } else if &command[..2] == "ls" {
            let lines = &command.lines().collect::<Vec<&str>>()[1..];
            for line in lines {
                let parts: Vec<&str> = line.split(' ').collect();
                if let Ok(value) = parts[0].parse::<usize>() {
                    for subpath in path.subpaths() {
                        let entry = tree.entry(subpath).or_insert(0);
                        *entry += value;
                    }
                }
            }
        }
    }

    let mut v1 = 0;
    let to_free = tree["/"] - 40000000;
    let mut v2 = usize::MAX;

    for value in tree.values() {
        if *value <= 100000 {
            v1 += value;
        } else if *value >= to_free && *value < v2 {
            v2 = *value;
        }
    }

    if two {
        v2
    } else {
        v1
    }
}

#[derive(Debug)]
struct Path {
    folders: Vec<String>,
}

impl Path {
    fn new() -> Self {
        Self {
            folders: vec![String::new()],
        }
    }

    fn push(&mut self, dir: &str) {
        self.folders.push(String::from(dir));
    }

    fn pop(&mut self) -> String {
        self.folders.pop().unwrap()
    }

    fn subpaths(&self) -> Vec<String> {
        let mut subpaths: Vec<String> = vec![];

        for i in 1..self.folders.len() + 1 {
            let mut path = String::new();
            for folder in &self.folders[..i] {
                path += folder;
                path += "/";
            }
            subpaths.push(path);
        }

        subpaths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 95437)
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 24933642)
    }
}
