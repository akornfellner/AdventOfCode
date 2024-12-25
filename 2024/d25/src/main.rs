use std::env::args;
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let elements = input.split("\n\n").map(Element::from).collect::<Vec<_>>();

    let keys = elements
        .iter()
        .filter(|e| matches!(e, Element::Key(_)))
        .collect::<Vec<_>>();
    let locks = elements
        .iter()
        .filter(|e| matches!(e, Element::Lock(_)))
        .collect::<Vec<_>>();

    let mut p1 = 0;

    for key in &keys {
        for lock in &locks {
            if find_key(key, lock) {
                p1 += 1;
            }
        }
    }

    (p1, 0)
}

fn find_key(key: &Element, lock: &Element) -> bool {
    match (key, lock) {
        (Element::Key(k), Element::Lock(l)) => {
            for i in 0..k.len() {
                if k[i] + l[i] > 5 {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

#[derive(Debug)]
enum Element {
    Key([usize; 5]),
    Lock([usize; 5]),
}

impl Element {
    fn from(s: &str) -> Self {
        let arr: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let mut numbers = [0; 5];

        for i in 0..5 {
            for c in &arr {
                if c[i] == '#' {
                    numbers[i] += 1;
                }
            }
            numbers[i] -= 1;
        }

        match arr[0][0] {
            '#' => Element::Lock(numbers),
            _ => Element::Key(numbers),
        }
    }
}
