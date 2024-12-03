use std::collections::HashMap;

use regex::Regex;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut mults: HashMap<usize, usize> = HashMap::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(&input) {
        let pos = cap.get(0).unwrap().start();
        let a: usize = cap[1].parse().unwrap();
        let b: usize = cap[2].parse().unwrap();
        mults.insert(pos, a * b);
    }

    let mut dos: HashMap<usize, bool> = HashMap::new();

    let re_do = Regex::new(r"do\(\)|don't\(\)").unwrap();
    for cap in re_do.captures_iter(&input) {
        let pos = cap.get(0).unwrap().start();
        let do_it = cap[0] == *"do()";
        dos.insert(pos, do_it);
    }

    let mut do_it = true;
    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..input.len() {
        if let Some(b) = dos.get(&i) {
            do_it = *b;
        }
        if let Some(mult) = mults.get(&i) {
            p1 += mult;
            if do_it {
                p2 += mult;
            }
        }
    }

    (p1, p2)
}
