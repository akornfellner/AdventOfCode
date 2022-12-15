use std::{collections::HashMap, fs};

pub fn solve(filename: &str, steps: u8) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let input: Vec<&str> = input.split("\n\n").collect();

    let poly = input[0];

    let lines: Vec<&str> = input[1].lines().collect();

    let mut pairs: Vec<String> = vec![];
    let mut next: HashMap<String, Vec<String>> = HashMap::new();
    let mut values: HashMap<String, usize> = HashMap::new();
    let mut letters: HashMap<String, char> = HashMap::new();

    for line in lines {
        let (pair, n, letter) = new_from_line(line);
        pairs.push(pair.clone());
        next.insert(pair.clone(), n);
        values.insert(pair.clone(), 0);
        letters.insert(pair, letter);
    }

    let poly: Vec<char> = poly.chars().collect();

    let mut letter_count: HashMap<char, usize> = HashMap::new();
    letter_count.insert(poly[0], 1);

    for i in 1..poly.len() {
        let pair = poly[i - 1].to_string() + &poly[i].to_string();
        add_value(&mut values, &pair, 1);
        if letter_count.contains_key(&poly[i]) {
            let v = letter_count[&poly[i]];
            let v = v + 1;
            letter_count.insert(poly[i], v);
        } else {
            letter_count.insert(poly[i], 1);
        }
    }

    for _ in 0..steps {
        let mut tmp = values.clone();
        for pair in &pairs {
            if values[pair] > 0 {
                let v = values[pair];
                add_value(&mut tmp, pair, -1 * v as i64);
                for n in &next[pair] {
                    add_value(&mut tmp, n, 1 * v as i64);
                }
                let letter = letters[pair];
                if letter_count.contains_key(&letter) {
                    let c = letter_count[&letter];
                    let c = c + v;
                    letter_count.insert(letter, c);
                } else {
                    letter_count.insert(letter, v);
                }
            }
        }
        values = tmp;
    }

    let mut min = std::usize::MAX;
    let mut max = 0;

    for (_, value) in letter_count {
        if value > max {
            max = value;
        }

        if value < min {
            min = value;
        }
    }

    max - min
}

fn new_from_line(line: &str) -> (String, Vec<String>, char) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let pair = parts[0].to_string();
    let chars: Vec<char> = parts[0].chars().collect();

    let mut new: Vec<String> = vec![];
    new.push(chars[0].to_string() + parts[1]);
    new.push(parts[1].to_owned() + &chars[1].to_string());

    (pair, new, parts[1].chars().collect::<Vec<char>>()[0])
}

fn add_value(values: &mut HashMap<String, usize>, pair: &str, value: i64) {
    let mut v = values[pair] as i64;
    v += value;
    let v = v as usize;
    values.insert(pair.to_string(), v);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn part_one_works() {
        assert_eq!(solve("input_test.txt", 10), 1588);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(solve("input_test.txt", 40), 2188189693529);
    }
}
