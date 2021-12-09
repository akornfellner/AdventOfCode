use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();

    let mut numbers: Vec<Vec<i32>> = vec![];

    for line in lines {
        let mut n: Vec<i32> = vec![];
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            n.push(c as i32 - 48);
        }
        numbers.push(n)
    }

    let mut visited: Vec<(usize, usize)> = vec![];

    let mut sizes: Vec<i32> = vec![];

    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            match get_size(i, j, &numbers, &mut visited) {
                Some(value) => {
                    sizes.push(value);
                }
                None => {}
            }
        }
    }

    sizes.sort();
    sizes.reverse();

    let mut result = 1;

    for size in &sizes[..3] {
        result *= size;
    }

    result
}

fn get_size(
    i: usize,
    j: usize,
    numbers: &[Vec<i32>],
    visited: &mut Vec<(usize, usize)>,
) -> Option<i32> {
    if visited.contains(&(i, j)) {
        return None;
    }

    visited.push((i, j));

    if numbers[i][j] == 9 {
        return None;
    }

    let rows = numbers.len();
    let cols = numbers[0].len();
    let mut starti = -1;
    let mut endi = 1;
    let mut startj = -1;
    let mut endj = 1;

    if i == 0 {
        starti = 0;
    } else if i == rows - 1 {
        endi = 0;
    }

    if j == 0 {
        startj = 0;
    } else if j == cols - 1 {
        endj = 0;
    }

    let mut sum = 1;

    for k in starti..endi + 1 {
        let index = (i as i32 + k) as usize;
        if numbers[index][j] != 9 && k != 0 {
            sum += match get_size(index, j, numbers, visited) {
                Some(value) => value,
                None => 0,
            };
        }
    }
    for k in startj..endj + 1 {
        let index = (j as i32 + k) as usize;
        if numbers[i][index] != 9 && k != 0 {
            sum += match get_size(i, index, numbers, visited) {
                Some(value) => value,
                None => 0,
            };
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 1134);
    }
}
