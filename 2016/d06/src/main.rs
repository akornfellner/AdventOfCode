use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(file: &str) -> (String, String) {
    let input = std::fs::read_to_string(file).unwrap();
    let cols = input.lines().next().unwrap().len();
    let mut letters: Vec<HashMap<char, usize>> = vec![HashMap::new(); cols];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let counter = letters[i].entry(c).or_insert(0);
            *counter += 1;
        }
    }

    let mut pairs: Vec<Vec<Pair>> = vec![];

    for letter in letters {
        let mut tmp: Vec<Pair> = vec![];
        for (k, v) in letter {
            tmp.push(Pair::new(k, v));
        }
        pairs.push(tmp);
    }

    let mut result1 = String::new();
    let mut result2 = String::new();

    for pair in &mut pairs {
        pair.sort();
        result1.push(pair[0].letter);
        result2.push(pair[pair.len() - 1].letter);
    }

    (result1, result2)
}

struct Pair {
    letter: char,
    count: usize,
}

impl Pair {
    fn new(letter: char, count: usize) -> Self {
        Pair { letter, count }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Eq for Pair {}
