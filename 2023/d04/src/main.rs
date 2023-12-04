use rustkorn::*;
use std::fs;

fn main() {
    let (p1, p2) = solve!();
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut winning: Vec<Set> = vec![];
    let mut own: Vec<Set> = vec![];

    for line in lines {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let mut winning_set = hashset!(i32);

        for number in parts[0].split_whitespace().skip(2) {
            winning_set.insert(number.parse::<i32>().unwrap());
        }

        let mut own_set = hashset!(i32);

        for number in parts[1].split_whitespace() {
            own_set.insert(number.parse::<i32>().unwrap());
        }

        winning.push(winning_set);
        own.push(own_set);
    }

    let mut p1 = 0;
    let mut p2: u32 = 0;

    let mut duplicates = hashmap!(usize, u32);

    for i in 0..winning.len() {
        let matches = winning[i].intersection(&own[i]).count() as u32;
        if matches > 0 {
            p1 += 2u32.pow(matches - 1);
        }

        let copies = *duplicates.entry(i).or_insert(1);

        for k in i + 1..=i + matches as usize {
            let entry = duplicates.entry(k).or_insert(1);
            *entry += copies;
        }
    }

    for i in 0..winning.len() {
        p2 += duplicates[&i];
    }

    (p1, p2)
}

type Set = std::collections::HashSet<i32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 13);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 30);
    }
}
