use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;

    for line in lines {
        result += proof_line(line);
    }
    result
}

fn proof_line(line: &str) -> usize {
    let braces = HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    let open = vec!['(', '{', '[', '<'];
    let points: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let chars: Vec<char> = line.chars().collect();

    let mut stack: Vec<char> = vec![];

    let mut sum = 0usize;

    for c in chars {
        if open.contains(&c) {
            stack.push(c)
        } else {
            let last = stack.pop().unwrap();
            let expect = braces[&last];
            if c != expect {
                sum += points[&c];
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 26397);
    }
}
