use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let tmp: Vec<&str> = line.split('-').collect();
        let s = tmp[0];
        let e = tmp[1];

        if caves.contains_key(s) {
            let mut n: Vec<&str> = caves[&s].clone();
            n.push(e);
            caves.insert(s, n);
        } else {
            caves.insert(s, vec![e]);
        }

        let tmp: Vec<&str> = line.split('-').collect();
        let e = tmp[0];
        let s = tmp[1];

        if caves.contains_key(s) {
            let mut n: Vec<&str> = caves[&s].clone();
            n.push(e);
            caves.insert(s, n);
        } else {
            caves.insert(s, vec![e]);
        }
    }

    let visited: Vec<&str> = vec![];

    let twice = false;

    visit("start", &caves, &visited, twice)
}

fn visit(cave: &str, caves: &HashMap<&str, Vec<&str>>, visited: &[&str], twice: bool) -> i32 {
    let mut visited = visited.to_owned();
    if cave == "end" {
        return 1;
    }

    let mut twice = twice;

    let mut result = 0;

    if is_small(cave) && visited.contains(&cave) {
        if !twice && cave != "start" {
            twice = true;
        } else {
            return 0;
        }
    }

    visited.push(cave);

    for n in &caves[cave] {
        result += visit(*n, caves, &visited, twice);
    }

    result
}

fn is_small(cave: &str) -> bool {
    if cave == cave.to_lowercase() {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 36);
    }
}
