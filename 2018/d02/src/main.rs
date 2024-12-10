use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, String) {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut two = 0;
    let mut three = 0;
    for line in input.lines() {
        let (t, th) = counts(line);
        if t {
            two += 1;
        }
        if th {
            three += 1;
        }
    }

    let mut common = String::new();

    for (i, line) in input.lines().enumerate() {
        for other in input.lines().skip(i + 1) {
            if let Some(c) = differ_by_one(line, other) {
                common = c;
                break;
            }
        }
    }

    (two * three, common)
}

fn counts(line: &str) -> (bool, bool) {
    let mut letters: HashMap<char, usize> = HashMap::new();

    for c in line.chars() {
        let count = letters.entry(c).or_insert(0);
        *count += 1;
    }

    let mut two = false;
    let mut three = false;

    for (_, count) in letters {
        if count == 2 {
            two = true;
        } else if count == 3 {
            three = true;
        }
    }

    (two, three)
}

fn differ_by_one(a: &str, b: &str) -> Option<String> {
    let mut diff = 0;
    for (ca, cb) in a.chars().zip(b.chars()) {
        if ca != cb {
            diff += 1;
        }
    }

    if diff == 1 {
        let common: String = a
            .chars()
            .zip(b.chars())
            .filter_map(|(ca, cb)| if ca == cb { Some(ca) } else { None })
            .collect();
        Some(common)
    } else {
        None
    }
}
