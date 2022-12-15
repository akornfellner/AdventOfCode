mod item;

use std::{cmp::Ordering, fs};

use item::Item;

pub fn solve(input: &str, two: bool) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let pairs: Vec<Pair> = input
        .split("\n\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .map(|pair| Pair::new(&pair))
        .collect();

    let mut result = 0;

    if !two {
        for (i, pair) in pairs.iter().enumerate() {
            if let Ordering::Less = pair.compare() {
                result += i + 1;
            }
        }

        return result;
    }

    let mut items: Vec<Item> = input.replace("\n\n", "\n").lines().map(Item::new).collect();

    items.push(Item::new("[[2]]"));
    items.push(Item::new("[[6]]"));

    items.sort();

    result = 1;

    for (i, item) in items.iter().enumerate() {
        if *item == Item::new("[[2]]") || *item == Item::new("[[6]]") {
            result *= i + 1;
        }
    }

    result
}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

impl Pair {
    fn new(pair: &[&str]) -> Self {
        let left = Item::new(pair[0]);
        let right = Item::new(pair[1]);

        Self { left, right }
    }

    fn compare(&self) -> Ordering {
        self.left.compare(&self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 13);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 140);
    }
}
