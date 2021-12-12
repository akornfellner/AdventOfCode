use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.split('\n').collect();

    let mut result = 0;

    for line in lines {
        let s: Vec<&str> = line.split(" | ").collect();
        let wires: Vec<String> = s[0].split(' ').map(|x| String::from(x)).collect();
        let digits: Vec<String> = s[1].split(' ').map(|x| String::from(x)).collect();

        let mapping = find_mapping(&wires);

        let digits: Vec<String> = digits.into_iter().map(|x| sort_chars(&x)).collect();

        let mut numbers: Vec<i32> = vec![];

        for digit in digits {
            numbers.push(mapping[&digit]);
        }

        let numbers: Vec<i32> = numbers.into_iter().rev().collect();
        let mut expo = 0;
        let mut sum = 0;
        let base: i32 = 10;

        for number in numbers {
            sum += number * base.pow(expo);
            expo += 1;
        }

        result += sum;
    }

    result
}

fn find_mapping(wires: &[String]) -> HashMap<String, i32> {
    let mut result: HashMap<i32, &str> = HashMap::new();
    let mut digits: HashMap<char, char> = HashMap::new();

    for w in wires {
        match w.len() {
            2 => {
                result.insert(1, w);
            }
            3 => {
                result.insert(7, w);
            }
            4 => {
                result.insert(4, w);
            }
            7 => {
                result.insert(8, w);
            }
            _ => (),
        }
    }

    for c in result[&7].chars() {
        if !result[&1].contains(c) {
            digits.insert('a', c);
        }
    }

    let c_f: Vec<char> = result[&1].chars().collect();

    for w in wires {
        let ch: Vec<char> = w.chars().collect();
        if ch.len() == 6 && !(ch.contains(&c_f[0]) && ch.contains(&c_f[1])) {
            if !ch.contains(&c_f[0]) {
                result.insert(6, w);
                digits.insert('c', c_f[0]);
                digits.insert('f', c_f[1]);
            } else {
                result.insert(6, w);
                digits.insert('c', c_f[0]);
                digits.insert('f', c_f[1]);
            }
        }
    }

    let chars_4: Vec<char> = result[&4].chars().collect();
    let mut chars: Vec<char> = vec![];

    for c in chars_4.clone() {
        if c != digits[&'c'] && c != digits[&'f'] {
            chars.push(c);
        }
    }

    for w in wires {
        if w.len() == 5 && w.contains(chars[0]) && w.contains(chars[1]) {
            result.insert(5, w);
        }
    }

    for w in wires {
        if w.len() == 5 && w.to_string() != result[&5] {
            if w.contains(digits[&'c']) && w.contains(digits[&'f']) {
                result.insert(3, w);
            } else {
                result.insert(2, w);
            }
        }
    }

    for w in wires {
        if w.len() == 6 && w.to_string() != result[&6] {
            if w.contains(chars_4[0])
                && w.contains(chars_4[1])
                && w.contains(chars_4[2])
                && w.contains(chars_4[3])
            {
                result.insert(9, w);
            } else {
                result.insert(0, w);
            }
        }
    }

    let mut sol: HashMap<String, i32> = HashMap::new();

    for (d, s) in result {
        sol.insert(sort_chars(s), d);
    }

    sol
}

fn sort_chars(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    let mut result = String::new();
    for c in chars {
        result += &c.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 61229);
    }
}
