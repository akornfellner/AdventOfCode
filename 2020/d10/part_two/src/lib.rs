use std::{collections::HashMap, fs};

pub fn solve(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();

    let mut numbers: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    numbers.push(0);

    let mut max = 0;

    for number in &numbers {
        if number > &max {
            max = *number;
        }
    }

    numbers.push(max + 3);

    numbers.sort_unstable();

    let mut values: Vec<usize> = vec![];

    for i in 0..numbers.len() {
        values.push(get_valid_followers(i, &numbers));
    }

    let mut visited: HashMap<usize, usize> = HashMap::new();

    get_paths(0, &values, &mut visited)
}

fn get_paths(index: usize, values: &[usize], visited: &mut HashMap<usize, usize>) -> usize {
    if values[index] == 0 {
        return 1;
    }

    if visited.contains_key(&index) {
        return visited[&index];
    }

    let mut result = 0;

    for i in 1..=values[index] {
        result += get_paths(index + i, values, visited);
    }

    visited.insert(index, result);

    result
}

fn get_valid_followers(index: usize, numbers: &[i32]) -> usize {
    let mut result = 0;

    for i in index + 1..numbers.len() {
        if numbers[i] - numbers[index] > 3 {
            break;
        }
        result += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 8);
    }
}
