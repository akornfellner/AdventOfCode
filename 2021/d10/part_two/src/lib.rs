use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut scores: Vec<usize> = vec![];

    for line in lines {
        if let Some(value) = proof_line(line) {
            scores.push(value);
        }
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn proof_line(line: &str) -> Option<usize> {
    let braces = HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    let open = vec!['(', '{', '[', '<'];
    let points: HashMap<char, usize> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let chars: Vec<char> = line.chars().collect();

    let mut stack: Vec<char> = vec![];

    for c in chars {
        if open.contains(&c) {
            stack.push(c)
        } else {
            let last = stack.pop().unwrap();
            let expect = braces[&last];
            if c != expect {
                return None;
            }
        }
    }

    let mut score = 0usize;

    stack.reverse();
    for c in stack {
        score *= 5;
        score += points[&c];
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 288957);
    }
}
