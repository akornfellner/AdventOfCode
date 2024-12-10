use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();

    let numbers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let p1 = numbers.iter().sum();

    let mut freqs = HashSet::new();
    let mut p2 = 0;

    'outer: loop {
        for n in &numbers {
            p2 += n;
            if freqs.contains(&p2) {
                break 'outer;
            }
            freqs.insert(p2);
        }
    }

    (p1, p2)
}
