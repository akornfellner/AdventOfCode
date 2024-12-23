use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env::args,
};
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

    let codes: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let pad = create_pad();
    let mut dp: HashMap<(Vec<char>, usize), usize> = HashMap::new();

    let mut p1 = 0;
    let mut p2 = 0;

    for code in &codes {
        let number = code
            .iter()
            .take(code.len() - 1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        p1 += count(code, &pad, 2, &mut dp) * number;
        p2 += count(code, &pad, 25, &mut dp) * number;
    }

    (p1, p2)
}

fn count(
    code: &[char],
    pad: &Pad,
    level: usize,
    dp: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    let mut c = 0;

    if dp.contains_key(&(code.to_vec(), level)) {
        return dp[&(code.to_vec(), level)];
    }

    let s = if code.contains(&'A') { 'A' } else { 'B' };

    for i in 0..code.len() {
        let s = if i == 0 { s } else { code[i - 1] };
        let paths = get_paths(s, code[i], pad);

        if level == 0 {
            c += paths.iter().map(|p| p.len()).min().unwrap();
        } else {
            c += paths
                .iter()
                .map(|p| count(p, pad, level - 1, dp))
                .min()
                .unwrap();
        }
    }

    dp.insert((code.to_vec(), level), c);

    c
}

fn is_valid(s: char, path: &[char], pad: &Pad) -> bool {
    let (mut x, mut y) = pad[&s];

    for c in path {
        match c {
            'v' => y -= 1,
            '^' => y += 1,
            '>' => x += 1,
            _ => x -= 1,
        }
        if (x, y) == (0, 0) || (x, y) == (0, 5) {
            return false;
        }
    }

    true
}

fn get_paths(s: char, e: char, pad: &Pad) -> Vec<Vec<char>> {
    let (sx, sy) = pad[&s];
    let (ex, ey) = pad[&e];
    let (dx, dy) = (ex as isize - sx as isize, ey as isize - sy as isize);

    let mut path = String::new();

    if dx > 0 {
        path.push_str(&">".repeat(dx as usize));
    } else {
        path.push_str(&"<".repeat(-dx as usize));
    }

    if dy > 0 {
        path.push_str(&"^".repeat(dy as usize));
    } else {
        path.push_str(&"v".repeat(-dy as usize));
    }

    let mut paths: HashSet<Vec<char>> = HashSet::new();

    for perm in Itertools::permutations(path.chars(), path.len()) {
        if is_valid(s, &perm, pad) {
            let mut p = perm.clone();
            p.push('B');
            paths.insert(p);
        }
    }

    paths.into_iter().collect()
}

type Pad = HashMap<char, (usize, usize)>;

fn create_pad() -> Pad {
    let mut pad: Pad = HashMap::new();
    pad.insert('0', (1, 0));
    pad.insert('A', (2, 0));
    pad.insert('1', (0, 1));
    pad.insert('2', (1, 1));
    pad.insert('3', (2, 1));
    pad.insert('4', (0, 2));
    pad.insert('5', (1, 2));
    pad.insert('6', (2, 2));
    pad.insert('7', (0, 3));
    pad.insert('8', (1, 3));
    pad.insert('9', (2, 3));
    pad.insert('<', (0, 4));
    pad.insert('v', (1, 4));
    pad.insert('>', (2, 4));
    pad.insert('^', (1, 5));
    pad.insert('B', (2, 5)); // because there can't be a second 'A' key i have used 'B' instead

    pad
}
