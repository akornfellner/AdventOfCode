use std::fs;

fn main() {
    println!("Part one: {}", solve("input.txt", false));
    println!("Part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> i32 {
    let lines = read_input(input);

    let mut count = 0;

    'outer: for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                let check = if two {
                    is_permutation(words[i], words[j])
                } else {
                    words[i] == words[j]
                };

                if check {
                    continue 'outer;
                }
            }
        }

        count += 1;
    }

    count
}

fn is_permutation(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();

    a_chars.sort();
    b_chars.sort();

    a_chars == b_chars
}

fn read_input(input: &str) -> Vec<String> {
    let input = fs::read_to_string(input).unwrap();
    input.lines().map(|s| s.to_string()).collect()
}
